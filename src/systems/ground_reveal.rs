use amethyst::core::specs::prelude::{Read, ReadStorage, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;

use entities::tile::LevelGrid;
use entities::tile::Tile;
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
        Write<'a, BinaryHeap<(Duration, Entity)>>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        Write<'a, TileUpdateQueue>
    );

    fn run(
        &mut self,
        (
            time,
            grid,
            mut heap,
            transforms,
            mut tiles,
            mut tile_update_queue
        ): Self::SystemData,
    ) {
        if let Some((mut reveal_time, mut entity)) = heap.peek().cloned() {
            while reveal_time <= time.absolute_time() {
                //the entity is to be revealed, so we delete it, but we already got the values by peeking
                heap.pop();

                let tran = transforms.get(entity).unwrap().clone();
                let x = tran.translation[0]  as i32;
                let y = - tran.translation[2] as i32;

                // reveal yourself
                tiles.get_mut(entity).unwrap().reveal();

                let mut neighbors = vec![];
                neighbors.extend(grid.direct_neighbors(x, y));

                for neighbor in neighbors.clone().iter() {
                    // add conceiled to queue
                    let tile = tiles.get_mut(*neighbor).unwrap();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            heap.push((
                                Duration::from_millis(500) + time.absolute_time(),
                                neighbor.clone(),
                            ));

                            let pos = neighbors.iter().position(|x| *x == *neighbor).unwrap();
                            neighbors.remove(pos);
                        }
                        _ => (),
                    }
                }

                neighbors.extend(grid.diagonal_neighbors(x, y));
                neighbors.push(entity);

                for neighbor in neighbors.drain(..) {
                    // add conceiled to queue
                    let tile = tiles.get_mut(neighbor).unwrap().clone();
                    match tile {
                        Tile::Ground { concealed: true } => (),
                        _ => {
                            let transform = transforms.get(neighbor).unwrap().clone();
                            let x = transform.translation[0] as usize;
                            let y = -transform.translation[2] as usize;
                            tile_update_queue.insert((x, y));
                        }
                    }
                }

                if let Some((new_reveal_time, new_entity)) = (heap.peek()).cloned() {
                    reveal_time = new_reveal_time;
                    entity = new_entity;
                } else {
                    return;
                };
            }
        }
    }
}
