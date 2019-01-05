use amethyst::{
    assets::Loader,
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entity, Read, ReadExpect, System, Write, WriteStorage},
    shrev::EventChannel,
};

use entities::Tile;
use eventhandling::{ClickHandlerComponent, HoverEvent, HoverHandlerComponent, Hovered};
use level::{LevelGrid, TilePatternMap};

use assetmanagement::util::{MeshStorages, TextureStorages};
use std::{cmp::Reverse, collections::BinaryHeap, time::Duration};

///
/// This system handles the recursive process of revealing tiles
/// including telling the adjacent tiles to update their meshes
///
pub struct GroundRevealSystem;

pub type RevealQueue = BinaryHeap<Reverse<(Duration, Entity)>>;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, Loader>,
        Write<'a, Hovered>,
        WriteStorage<'a, HoverHandlerComponent>,
        Write<'a, EventChannel<HoverEvent>>,
        WriteStorage<'a, ClickHandlerComponent>,
        Read<'a, TilePatternMap>,
        Read<'a, LevelGrid>,
        Write<'a, Option<RevealQueue>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        MeshStorages<'a>,
        TextureStorages<'a>,
    );

    fn run(
        &mut self,
        (
            time,
            loader,
            mut hover,
            mut hovers,
            mut hover_channel,
            mut clickers,
            dict,
            level_grid,
            mut ground_reveal_queue,
            mut transforms,
            mut tiles,
            mut mesh_storages,
            mut texture_storages,
        ): Self::SystemData,
    ) {
        if let Some(ref mut ground_reveal_queue) = *ground_reveal_queue {
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

                            level_grid.update_tile(
                                x,
                                y,
                                &dict,
                                &mut transforms,
                                &tiles,
                                &mut texture_storages,
                                &mut mesh_storages,
                                &mut hover,
                                &loader,
                                &mut hover_channel,
                                &mut hovers,
                                &mut clickers,
                            );
                        }
                    }
                }
            }
        } else {
            error!("reveal_system is running without a valid reveal queue!");
        }
    }
}
