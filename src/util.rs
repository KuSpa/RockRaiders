use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::storage::GenericWriteStorage;
use amethyst::prelude::*;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, MeshHandle, Texture};
use amethyst::shred::{FetchMut, Fetch};
use assetloading::asset_loader::AssetManager;

/// rotates the array clockwise
pub fn rotate_3x3<T: Clone>(input: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    let mut result = input.clone();

    for x in 0..3 {
        for y in 0..3 {
            result[x][y] = input[3 - y - 1][x].clone()
        }
    }
    result
}

//pub fn gather_storages<
//    M: GenericWriteStorage<Component = MeshHandle>,
//    T: GenericWriteStorage<Component = Material>,
//>(
//    world: &World,
//) -> (
//    FetchMut<Loader>,
//    (
//        FetchMut<AssetManager<Mesh>>,
//        M,
//        FetchMut<AssetStorage<Mesh>>,
//    ),
//    (
//        FetchMut<AssetManager<Texture>>,
//        T,
//        FetchMut<AssetStorage<Texture>>,
//        Fetch<MaterialDefaults>,
//    ),
//) {
//    (
//        world.write_resource::<Loader>(),
//        (
//            world.write_resource::<AssetManager<Mesh>>(),
//            world.write_storage::<MeshHandle>(),
//            world.write_resource::<AssetStorage<Mesh>>(),
//        ),
//        (
//            world.write_resource::<AssetManager<Texture>>(),
//            world.write_resource::<Material>(),
//            world.write_resource::<AssetStorage<Texture>>(),
//            world.read_resource::<MaterialDefaults>(),
//        ),
//    )
//}

fn get_mesh_handle_storage<'a, M: GenericWriteStorage<Component = MeshHandle>>(world: &'a World) -> &'a M{
    &world.write_storage::<MeshHandle>()
}

//pub fn insert_into_storages(
//    Loader,
//    (
//        AssetManager<Mesh>,
//        MeshHandle,
//        AssetStorage<Mesh>,
//    ),
//    (
//        AssetManager<Texture>,
//        WriteStorage<Material>,
//        AssetStorage<Texture>,
//        MaterialDefaults,
//    ),
//);
//) {
//
//let mesh = {
//mesh_manager.get_asset_handle_or_load(
//& classifier,
//ObjFormat,
//Default::default(),
//& mut mesh_storage,
//&amethyst_loader,
//)
//};
//
//// load texture/material
//let material = {
//let handle = tex_manager.get_asset_handle_or_load(
//& classifier,
//PngFormat,
//TextureMetadata::srgb(),
//& mut tex_storage,
//& amethyst_loader,
//);
//Material {
//albedo: handle,
//..mat_defaults.0.clone()
//}
//};
//
//mat_storage.insert(entity, material).unwrap();
//mesh_handles.insert(entity, mesh).unwrap();
//}
