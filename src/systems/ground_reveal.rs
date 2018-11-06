use amethyst::core::specs::prelude::{Read, ReadExpect, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use entities::tile::LevelGrid;
use entities::tile::Tile;
use std::collections::BinaryHeap;
use std::time::Duration;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Vector3};
use assetloading::asset_loader::AssetManager;

///
/// This system handles the recursive process of revealing tiles
/// including telling the adjacent tiles to update their meshes
///
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
        (
            time,
            loader,
            grid,
            dict,
            mut texture_manager,
            mut mesh_manager,
            mut heap,
            mut meshhandles,
            mut meshes,
            mut materials,
            mut textures,
            mut transforms,
            mut tiles,
            defaults,
        ): Self::SystemData,
    ) {
        if let Some((mut reveal_time, mut entity)) = heap.peek().cloned() {
            while reveal_time <= time.absolute_time() {
                warn!("entering {:?} is big", reveal_time);
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

                // TODO maybe have an own system for update meshes
                for neighbor in neighbors.drain(..) {
                    // add conceiled to queue
                    let tile = tiles.get_mut(neighbor).unwrap().clone();
                    match tile {
                        Tile::Ground { concealed: true } => (),
                        _ => {
                            let transform = transforms.get(neighbor).unwrap().clone();
                            let x = transform.translation[0] as usize;
                            let y = -transform.translation[2] as usize;
                            warn!("{:?}", (x, y));

                            let (wall_type, wall_rotation) =
                                grid.determine_sprite_for(x, y, &dict, &tiles);
                            //TODO update old transforms rotation
                            let mut new_transform = Transform::default();
                            new_transform.set_position(Vector3 {
                                x: x as f32,
                                y: 0.0,
                                z: -(y as f32),
                            });

                            //add rotation to local transform
                            new_transform.rotate_local(
                                Vector3::new(0.0, 1.0, 0.0),
                                Deg(wall_rotation as f32),
                            );

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

                            meshhandles.insert(neighbor, mesh).unwrap();
                            materials.insert(neighbor, material).unwrap();
                            transforms.insert(neighbor, new_transform).unwrap();
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
