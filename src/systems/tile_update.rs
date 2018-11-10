use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::specs::prelude::{Read, ReadExpect, ReadStorage, System, Write, WriteStorage};
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use assetloading::asset_loader::AssetManager;
use entities::tile::{LevelGrid, Tile};

pub struct TileUpdateSystem;

impl<'a> System<'a> for TileUpdateSystem {
    type SystemData = (
        Write<'a, TileUpdateQueue>,
        Read<'a, Vec<([[Tile; 3]; 3], String)>>,
        Read<'a, LevelGrid>,
        ReadStorage<'a, Tile>,
        ReadExpect<'a, Loader>,
        WriteStorage<'a, Transform>,
        (
            Write<'a, AssetManager<Mesh>>,
            WriteStorage<'a, MeshHandle>,
            Write<'a, AssetStorage<Mesh>>,
        ),
        (
            Write<'a, AssetManager<Texture>>,
            WriteStorage<'a, Material>,
            Write<'a, AssetStorage<Texture>>,
            ReadExpect<'a, MaterialDefaults>,
        ),
    );

    fn run(
        &mut self,
        (
            mut tiles,
            dict,
            level_grid,
            tile_storage,
            amethyst_loader,
            mut transform_storage,
            (mut mesh_manager, mut mesh_handles, mut mesh_storage),
            (mut tex_manager, mut mat_storage, mut tex_storage, mat_defaults),
        ): Self::SystemData,
    ) {
        if tiles.is_empty() {
            return;
        };

        let tile_grid = level_grid.generate_tile_grid_copy(&tile_storage);

        for (x, y) in tiles.clone().into_iter() {
            // get sprite definition
            let (wall_type, wall_rotation) =
                tile_grid.determine_sprite_for(x, y, &dict);

            let mut transform = Transform::default();
            // maybe set 00 top left instead of bottom left?
            transform.set_position(Vector3 {
                x: x as f32,
                y: 0.0,
                z: -(y as f32),
            });
            transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(wall_rotation as f32));

            // load mesh
            let mesh = {
                mesh_manager.get_asset_handle_or_load(
                    &wall_type,
                    ObjFormat,
                    Default::default(),
                    &mut mesh_storage,
                    &amethyst_loader,
                )
            };

            // load texture/material
            let material = {
                let handle = tex_manager.get_asset_handle_or_load(
                    &wall_type,
                    PngFormat,
                    TextureMetadata::srgb(),
                    &mut tex_storage,
                    &amethyst_loader,
                );
                Material {
                    albedo: handle,
                    ..mat_defaults.0.clone()
                }
            };

            let entity = level_grid.get(x as i32, y as i32).unwrap();
            transform_storage.insert(entity, transform).unwrap();
            mat_storage.insert(entity, material).unwrap();
            mesh_handles.insert(entity, mesh).unwrap();
        }

        // all tiles have been updated, we can remove the requests from the list
        tiles.clear()
    }
}

///
/// Wrapper for a Vec of Tiles that needs to be updated
///
pub struct TileUpdateQueue {
    tiles: Vec<(i32, i32)>,
}

impl std::ops::Deref for TileUpdateQueue {
    type Target = Vec<(i32, i32)>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl std::ops::DerefMut for TileUpdateQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}


impl Default for TileUpdateQueue {
    fn default() -> Self {
        TileUpdateQueue {
            tiles: Vec::<(i32, i32)>::new(),
        }
    }
}