use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::{Entity, ReadExpect, Write, WriteStorage};
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use assetmanagement::AssetManager;
use collision::primitive::Primitive3;

pub type AssetStorages<'a> = (
    ReadExpect<'a, Loader>,
    Write<'a, AssetManager<Mesh>>,
    WriteStorage<'a, MeshHandle>,
    Write<'a, AssetStorage<Mesh>>,
    Write<'a, AssetManager<Texture>>,
    WriteStorage<'a, Material>,
    Write<'a, AssetStorage<Texture>>,
    ReadExpect<'a, MaterialDefaults>,
);

pub fn insert_into_asset_storages(entity: Entity, asset_name: &str, storages: &mut AssetStorages) {
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

use systems::HoverHandler;

pub fn add_hover_handler<'a>(
    entity: Entity,
    asset_name: &str,
    bounding_box: Primitive3<f32>,
    (loader, tex_manager, mat_storage, tex_storage, default_mat, hover_storage): &mut (
        ReadExpect<'a, Loader>,
        Write<'a, AssetManager<Texture>>,
        WriteStorage<'a, Material>,
        Write<'a, AssetStorage<Texture>>,
        ReadExpect<'a, MaterialDefaults>,
        WriteStorage<'a, HoverHandler>,
    ),
) {
    let hover_mat = {
        let handle = tex_manager.get_asset_handle_or_load(
            &[asset_name, "_hover"].join(""),
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
    let handler = HoverHandler {
        hover: hover_mat,
        bounding_box,
    };
    hover_storage.insert(entity, handler);
}
