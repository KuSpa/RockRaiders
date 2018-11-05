use amethyst::core::specs::prelude::{Read, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::renderer::{MeshHandle, TextureHandle};
use entities::tile::Tile;
use std::collections::BinaryHeap;
use std::time::Duration;
use entities::tile::LevelGrid;

pub struct GroundRevealSystem;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, LevelGrid>,
        Write<'a, BinaryHeap<(Duration, Entity)>>,
        WriteStorage<'a, MeshHandle>,
        WriteStorage<'a, TextureHandle>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
    );

    fn run(
        &mut self,
        (time, grid, mut heap, mut meshes, mut textures, mut transforms, mut tiles): Self::SystemData,
    ) {
        if let Some((reveal_time, entity)) = heap.peek().cloned() {
            while reveal_time >= time.absolute_time() {
                //the entity is to be revealed, so we delete it, but we already got the values by peeking
                heap.pop();


                let tran= transforms.get(entity).unwrap();
                let x = tran.translation[0] as i32;
                let y = tran.translation[1] as i32;


                // reveal yourself
                tiles.get_mut(entity).unwrap().reveal();


                let mut neighbors = vec![entity.clone()];
                neighbors.extend(grid.direct_neighbors(x,y));


                for neighbor in neighbors.clone().iter() {
                    // add conceiled to queue
                    let tile = tiles.get_mut(*neighbor).unwrap();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            heap.push((Duration::from_millis(500) + reveal_time, neighbor.clone()));
                            let pos = neighbors.iter().position(|x| *x == *neighbor).unwrap();
                            neighbors.remove(pos);
                        }
                        _ => (),
                    }
                }

                neighbors.extend(grid.diagonal_neighbors(x,y));


                for neighbor in neighbors.iter() {
                    // add conceiled to queue
                    let tile = tiles.get_mut(*neighbor).unwrap();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            ()
                        }
                        _ => {
                            //TODO update meshes
                        },
                    }
                }

                if let Some((reveal_time, entity)) = heap.peek().cloned() {} else {
                    return;
                };
            }
        }
    }
}
