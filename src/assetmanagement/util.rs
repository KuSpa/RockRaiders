use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::{Entity, ReadExpect, Write, WriteStorage};
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use assetmanagement::AssetManager;

type Storages<'a> = (
    ReadExpect<'a, Loader>,
    Write<'a, AssetManager<Mesh>>,
    WriteStorage<'a, MeshHandle>,
    Write<'a, AssetStorage<Mesh>>,
    Write<'a, AssetManager<Texture>>,
    WriteStorage<'a, Material>,
    Write<'a, AssetStorage<Texture>>,
    ReadExpect<'a, MaterialDefaults>,
);

pub fn insert_into_storages(entity: Entity, asset_name: &str, storages: &mut Storages) {
    let (
        ref loader,
        ref mut mesh_manager,
        ref mut mesh_handles,
        ref mut mesh_storage,
        ref mut tex_manager,
        ref mut mat_storage,
        ref mut tex_storage,
        ref default_mat,
    ) = storages;

    let mesh = {
        mesh_manager.get_asset_handle_or_load(
            asset_name,
            ObjFormat,
            Default::default(),
            mesh_storage,
            &loader,
        )
    };

    // load texture/material
    let material = {
        let handle = tex_manager.get_asset_handle_or_load(
            asset_name,
            PngFormat,
            TextureMetadata::srgb(),
            tex_storage,
            &loader,
        );
        Material {
            albedo: handle,
            ..default_mat.0.clone()
        }
    };

    mat_storage.insert(entity, material).unwrap();
    mesh_handles.insert(entity, mesh).unwrap();
}
