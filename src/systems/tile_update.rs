use amethyst::core::specs::prelude::{Read, ReadExpect, ReadStorage, System, Write, WriteStorage};
use amethyst::core::transform::Transform;
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use entities::tile::{Tile, LevelGrid};
use std::vec::IntoIter;
use assetloading::asset_loader::AssetManager;
use amethyst::assets::{AssetStorage, Loader};

pub struct TileUpdateSystem {
    pub dict: Vec<([[Tile; 3]; 3], String)>
}

impl<'a> System<'a> for TileUpdateSystem {
    type SystemData =
    (Write<'a, TileUpdateQueue>,
     Read<'a, LevelGrid>,
     ReadStorage<'a, Tile>,
     ReadExpect<'a, Loader>,
     WriteStorage<'a, Transform>,
     (Write<'a, AssetManager<Mesh>>, WriteStorage<'a, MeshHandle>, Write<'a, AssetStorage<Mesh>>, ),
     (Write<'a, AssetManager<Texture>>, WriteStorage<'a, Material>,Write<'a, AssetStorage<Texture>>, ReadExpect<'a, MaterialDefaults>, )
    )
    ;

    fn run(
        &mut self,
        (mut tiles,
            level_grid,
            tile_storage,
            amethyst_loader,
            mut transform_storage,
            (mut mesh_manager, mut mesh_handles, mut mesh_storage),
            (mut tex_manager, mut mat_storage, mut tex_storage, mat_defaults)): Self::SystemData,
    ) {
        if tiles.is_empty() { return; };

        // TODO setup stuff
        // TODO create a grid here

        for (x, y) in tiles.clone().into_iter() {

            // get sprite defintion
            let (wall_type, wall_rotation) = level_grid.determine_sprite_for(x, y, &self.dict, &tile_storage);

            let mut transform = Transform::default();
            // maybe set 00 top left instead of bottom left?
            transform.set_position(Vector3 { x: x as f32, y: 0.0, z: -(y as f32) });
            transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(wall_rotation as f32));

            let mesh_path = format!("meshes/{}.obj", wall_type);
            let texture_path = format!("textures/{}.png", wall_type);

            // load mesh
            let mesh = {
                mesh_manager.get_asset_handle_or_load(
                    &mesh_path,
                    ObjFormat,
                    Default::default(),
                    &mut mesh_storage,
                    &amethyst_loader,
                )
            };

            // load texture/material
            let material = {
                let handle = tex_manager.get_asset_handle_or_load(
                    &texture_path,
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

            // write into storages
            let entity = level_grid.get(x as i32, y as i32).unwrap();
            transform_storage.insert(entity, transform).unwrap();
            mat_storage.insert(entity, material).unwrap();
            mesh_handles.insert(entity, mesh).unwrap();
        };

        // all tiles have been updated, we can remove the requests from the list
        tiles.clear()
    }
}


///
/// Wrapper for a Vec of Tiles that needs to be updated
///
#[derive(Clone)]
pub struct TileUpdateQueue {
    tiles: Vec<(usize, usize)>
}

impl TileUpdateQueue {
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    pub fn insert(&mut self, item: (usize, usize)) {
        self.tiles.push(item);
    }

    pub fn clear(&mut self) {
        self.tiles.clear()
    }
}

impl Default for TileUpdateQueue {
    fn default() -> Self {
        TileUpdateQueue { tiles: Vec::<(usize, usize)>::new() }
    }
}

impl IntoIterator for TileUpdateQueue {
    type Item = (usize, usize);
    type IntoIter = IntoIter<(usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_iter()
    }
}