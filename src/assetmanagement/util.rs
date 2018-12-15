use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Entity, ReadExpect, Write, WriteStorage},
    renderer::{
        Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture,
        TextureMetadata,
    },
};

use assetmanagement::AssetManager;

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

use ncollide3d::shape::Shape;
use systems::HoverHandler;
pub fn add_hover_handler<'a>(
    entity: Entity,
    asset_name: &str,
    bounding_box: Box<dyn Shape<f32>>,
    (loader, tex_manager, tex_storage, hover_storage): &mut (
        ReadExpect<'a, Loader>,
        Write<'a, AssetManager<Texture>>,
        Write<'a, AssetStorage<Texture>>,
        WriteStorage<'a, HoverHandler>,
    ),
) {
    let hover_mat = tex_manager.get_asset_handle_or_load(
        &[asset_name, "_hover"].join(""),
        PngFormat,
        TextureMetadata::srgb(),
        tex_storage,
        &loader,
    );
    let handler = HoverHandler {
        hover: hover_mat,
        bounding_box,
    };
    hover_storage.insert(entity, handler).unwrap();
}
