use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, Read, ReadExpect, System, Write, WriteStorage};
use amethyst::renderer::{Material, MaterialDefaults, Mesh, MeshHandle, Texture};

use entities::Tile;
use level::{LevelGrid, TilePatternMap};

use assetmanagement::AssetManager;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Duration;

///
/// This system handles the recursive process of revealing tiles
/// including telling the adjacent tiles to update their meshes
///
pub struct GroundRevealSystem;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, TilePatternMap>,
        Read<'a, LevelGrid>,
        Write<'a, BinaryHeap<Reverse<(Duration, Entity)>>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        (
            ReadExpect<'a, Loader>,
            Write<'a, AssetManager<Mesh>>,
            WriteStorage<'a, MeshHandle>,
            Write<'a, AssetStorage<Mesh>>,
            Write<'a, AssetManager<Texture>>,
            WriteStorage<'a, Material>,
            Write<'a, AssetStorage<Texture>>,
            ReadExpect<'a, MaterialDefaults>,
        ),
    );

    fn run(
        &mut self,
        (time, dict, level_grid, mut ground_reveal_queue, mut transforms, mut tiles, mut storages): Self::SystemData,
    ) {
        while !ground_reveal_queue.is_empty()
            && ((ground_reveal_queue.peek().unwrap().0).0 <= time.absolute_time())
        {
            let Reverse((_, entity)) = ground_reveal_queue.pop().unwrap();

            // reveal yourself
            if !tiles.get_mut(entity).unwrap().reveal() {
                break;
            };

            let tran = transforms.get(entity).unwrap().clone();
            let x = tran.translation().x as i32;
            let y = tran.translation().z as i32;

            let mut neighbors = vec![];
            neighbors.extend(level_grid.direct_neighbors(x, y));

            for neighbor in neighbors.clone().iter() {
                // add concealed to queue
                let tile = tiles.get_mut(*neighbor).unwrap();
                match tile {
                    Tile::Ground { concealed: true } => {
                        ground_reveal_queue.push(Reverse((
                            Duration::from_millis(50) + time.absolute_time(),
                            *neighbor,
                        )));

                        let pos = neighbors.iter().position(|x| *x == *neighbor).unwrap();
                        neighbors.remove(pos);
                    }
                    _ => (),
                }
            }

            neighbors.extend(level_grid.diagonal_neighbors(x, y));
            neighbors.push(entity);

            for neighbor in neighbors.drain(..) {
                // add concealed to queue
                let tile = tiles.get_mut(neighbor).unwrap().clone();
                match tile {
                    Tile::Ground { concealed: true } => (),
                    _ => {
                        let transform = transforms.get(neighbor).unwrap().clone();
                        let x = transform.translation().x as i32;
                        let y = transform.translation().z as i32;

                        level_grid.update_tile(x, y, &dict, &mut transforms, &tiles, &mut storages);
                    }
                }
            }
        }
    }
}
