use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::storage::GenericWriteStorage;
use amethyst::ecs::{Entity, WriteStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use amethyst::shred::{Fetch, FetchMut};
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

pub fn insert_into_storages<M, T>(
    entity: Entity,
    asset_name: &str,
    loader: &Loader,
    // mesh stuff
    mesh_manager: &mut AssetManager<Mesh>,
    mesh_handles: &mut M,
    mesh_storage: &mut AssetStorage<Mesh>,

    //material
    tex_manager: &mut AssetManager<Texture>,
    mat_storage: &mut T,
    tex_storage: &mut AssetStorage<Texture>,
    default_mat: Material,
) where
    M: GenericWriteStorage<Component = MeshHandle>,
    T: GenericWriteStorage<Component = Material>,
{
    let mesh = {
        mesh_manager.get_asset_handle_or_load(
            asset_name,
            ObjFormat,
            Default::default(),
            mesh_storage,
            loader,
        )
    };

    // load texture/material
    let material = {
        let handle = tex_manager.get_asset_handle_or_load(
            asset_name,
            PngFormat,
            TextureMetadata::srgb(),
            tex_storage,
            loader,
        );
        Material {
            albedo: handle,
            ..default_mat
        }
    };

    mat_storage.insert(entity, material).unwrap();
    mesh_handles.insert(entity, mesh).unwrap();
}

pub fn gather_storages(
    world: &World,
) -> (
    FetchMut<Loader>,
    // mesh stuff
    FetchMut<AssetManager<Mesh>>,
    WriteStorage<MeshHandle>,
    FetchMut<AssetStorage<Mesh>>,
    //material
    FetchMut<AssetManager<Texture>>,
    WriteStorage<Material>,
    FetchMut<AssetStorage<Texture>>,
    Material,
) {
    (
        world.write_resource::<Loader>(),
        world.write_resource::<AssetManager<Mesh>>(),
        world.write_storage::<MeshHandle>(),
        world.write_resource::<AssetStorage<Mesh>>(),
        world.write_resource::<AssetManager<Texture>>(),
        world.write_storage::<Material>(),
        world.write_resource::<AssetStorage<Texture>>(),
        world.read_resource::<MaterialDefaults>().0.clone(),
    )
}
