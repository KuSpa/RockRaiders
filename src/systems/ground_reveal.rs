use amethyst::core::specs::prelude::{Read, ReadStorage, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;

use entities::tile::LevelGrid;
use entities::tile::Tile;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Duration;
use systems::TileUpdateQueue;

///
/// This system handles the recursive process of revealing tiles
/// including telling the adjacent tiles to update their meshes
///
pub struct GroundRevealSystem;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, LevelGrid>,
        Write<'a, BinaryHeap<Reverse<(Duration, Entity)>>>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        Write<'a, TileUpdateQueue>,
    );

    fn run(
        &mut self,
        (time, grid, mut heap, transforms, mut tiles, mut tile_update_queue): Self::SystemData,
    ) {
        while !heap.is_empty() && ((heap.peek().unwrap().0).0 <= time.absolute_time()) {
            let Reverse((_, entity)) = heap.pop().unwrap();

            let tran = transforms.get(entity).unwrap().clone();
            let x = tran.translation[0] as i32;
            let y = -tran.translation[2] as i32;

            // reveal yourself
            tiles.get_mut(entity).unwrap().reveal();

            let mut neighbors = vec![];
            neighbors.extend(grid.direct_neighbors(x, y));

            for neighbor in neighbors.clone().iter() {
                // add conceiled to queue
                let tile = tiles.get_mut(*neighbor).unwrap();
                match tile {
                    Tile::Ground { concealed: true } => {
                        heap.push(Reverse((
                            Duration::from_millis(200) + time.absolute_time(),
                            *neighbor,
                        )));

                        let pos = neighbors.iter().position(|x| *x == *neighbor).unwrap();
                        neighbors.remove(pos);
                    }
                    _ => (),
                }
            }

            neighbors.extend(grid.diagonal_neighbors(x, y));
            neighbors.push(entity);

            for neighbor in neighbors.drain(..) {
                // add concealed to queue
                let tile = tiles.get_mut(neighbor).unwrap().clone();
                match tile {
                    Tile::Ground { concealed: true } => (),
                    _ => {
                        let transform = transforms.get(neighbor).unwrap().clone();
                        let x = transform.translation[0] as i32;
                        let y = -transform.translation[2] as i32;
                        tile_update_queue.push((x, y));
                    }
                }
            }
        }
    }
}
