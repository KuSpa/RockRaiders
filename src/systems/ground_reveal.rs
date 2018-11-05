use amethyst::core::specs::prelude::{Read, ReadExpect, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, storage::GenericReadStorage};
use amethyst::renderer::{TextureMetadata, MaterialDefaults,PngFormat,Material,Texture, Mesh,MeshHandle, ObjFormat, TextureHandle};
use entities::tile::Tile;
use std::collections::BinaryHeap;
use std::time::Duration;
use entities::tile::LevelGrid;
use assetloading::asset_loader::AssetManager;
use amethyst::assets::{Loader, AssetStorage};

pub struct GroundRevealSystem;

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, Loader>,
        Read<'a, LevelGrid>,
        Read<'a, Vec<([[Tile; 3]; 3], String)>>,
        Write<'a, AssetManager<Texture>>,
        Write<'a, AssetManager<Mesh>>,
        Write<'a, BinaryHeap<(Duration, Entity)>>,
        WriteStorage<'a, MeshHandle>,
        Write<'a, AssetStorage<Mesh>>,
        WriteStorage<'a, Material>,
        Write<'a, AssetStorage<Texture>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tile>,
        ReadExpect<'a, MaterialDefaults>,
    );

    fn run(
        &mut self,
        (time, loader, grid, dict, mut texture_manager, mut mesh_manager, mut heap, mut meshhandles,mut meshes, mut materials,mut textures, mut transforms, mut tiles, defaults): Self::SystemData,
    ) {
        if let Some((mut reveal_time, mut entity)) = heap.peek().cloned() {
            while reveal_time >= time.absolute_time() {

                warn!("entering {:?}", entity);
                //the entity is to be revealed, so we delete it, but we already got the values by peeking
                heap.pop();


                let tran = transforms.get(entity).unwrap().clone();
                let x = tran.translation[0] as i32;
                let y = tran.translation[1] as i32;


                // reveal yourself
                tiles.get_mut(entity).unwrap().reveal();


                let mut neighbors = vec![];
                neighbors.extend(grid.direct_neighbors(x, y));


                for neighbor in neighbors.clone().iter() {
                    // add conceiled to queue
                    let tile = tiles.get_mut(*neighbor).unwrap();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            heap.push((Duration::from_millis(1000) + reveal_time, neighbor.clone()));

                            let pos = neighbors.iter().position(|x| *x == *neighbor).unwrap();
                            neighbors.remove(pos);
                        }
                        _ => (),
                    }
                }

                neighbors.extend(grid.diagonal_neighbors(x, y));
                neighbors.push(entity);


                // TODO maybe have an own system for update meshes
                for neighbor in neighbors.drain(..) {
                    // add conceiled to queue
                    let tile = tiles.get_mut(neighbor).unwrap().clone();
                    match tile {
                        Tile::Ground { concealed: true } => {
                            ()
                        }
                        _ => {
                            let mut transform = transforms.get_mut(entity).unwrap();
                            let x = transform.translation[0] as usize;
                            let y = transform.translation[1] as usize;

                            let (wall_type, rotation) = grid.determine_sprite_for(x, y, &dict, &tiles);
                            //TODO
                            // transform.set_rotation()

                            let mesh_path = format!("meshes/{}.obj", wall_type);
                            let texture_path = format!("textures/{}.png", wall_type);

                            let material = {
                                let handle = texture_manager.get_asset_handle_or_load(
                                    &texture_path,
                                    PngFormat,
                                    TextureMetadata::srgb(),
                                    &mut textures,
                                    &loader,
                                );
                                Material {
                                    albedo: handle,
                                    ..defaults.0.clone()
                                }
                            };

                            let mesh = {
                                mesh_manager.get_asset_handle_or_load(
                                    &mesh_path,
                                    ObjFormat,
                                    Default::default(),
                                    &mut meshes,
                                    &loader,
                                )
                            };

                            meshhandles.insert(neighbor, mesh);
                            materials.insert(neighbor, material);
                            // we got transform mutable and already changed it.
                        }
                    }
                }

                if let Some((new_reveal_time, new_entity)) = (heap.peek()).cloned() {
                    reveal_time = new_reveal_time;
                    entity = new_entity;
                } else {
                    warn!("break");
                    return;
                };
            }
        }
    }
}
