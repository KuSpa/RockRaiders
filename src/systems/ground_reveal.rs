use amethyst::core::specs::prelude::{Read, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::renderer::{MeshHandle, TextureHandle};
use entities::tile::Tile;
use std::collections::BinaryHeap;
use std::time::Duration;

pub struct GroundRevealSystem;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        Write<'a, BinaryHeap<(Duration, Entity)>>,
        WriteStorage<'a, MeshHandle>,
        WriteStorage<'a, TextureHandle>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
    );

    fn run(
        &mut self,
        (time, mut heap, mut meshes, mut textures, mut transforms, mut tiles): Self::SystemData,
    ) {
        if let Some((reveal_time, entity)) = heap.peek().cloned() {
            while reveal_time >= time.absolute_time() {
                //the entity is to be revealed, so we delete it, but we already got the values by peeking
                heap.pop();

                // reveal yourself
                tiles.get_mut(entity).unwrap().reveal();

                let neightbors = vec![entity.clone()];
                // TODO get neighbors

                for neighbor in neightbors.iter() {
                    // add conceiled to queue
                    let tile = tiles.get_mut(*neighbor).unwrap();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            heap.push((Duration::from_millis(500) + reveal_time, neighbor.clone()))
                        } // TODO refactor 500 into const maybe
                        _ => {
                            // TODO update meshes on remaining (including self)}
                        }
                    }
                }

                if let Some((reveal_time, entity)) = heap.peek().cloned() {
                } else {
                    return;
                };
            }
        }
    }
}
