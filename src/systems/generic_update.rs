use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::specs::prelude::{ ReadExpect, System, Write, WriteStorage};
use amethyst::ecs::prelude::Entity;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use assetloading::asset_loader::AssetManager;

pub struct GenericAssetUpdateSystem;

impl<'a> System<'a> for GenericAssetUpdateSystem {
    type SystemData = (
        Write<'a, UpdateQueue<'a>>,
        ReadExpect<'a, Loader>,
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
            entities,
            amethyst_loader,
            (mut mesh_manager, mut mesh_handles, mut mesh_storage),
            (mut tex_manager, mut mat_storage, mut tex_storage, mat_defaults),
        ): Self::SystemData,
    ) {
        for (entity, classifier) in entities.drain(..) {

            // load mesh
            let mesh = {
                mesh_manager.get_asset_handle_or_load(
                    &classifier,
                    ObjFormat,
                    Default::default(),
                    &mut mesh_storage,
                    &amethyst_loader,
                )
            };

            // load texture/material
            let material = {
                let handle = tex_manager.get_asset_handle_or_load(
                    &classifier,
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

            mat_storage.insert(entity, material).unwrap();
            mesh_handles.insert(entity, mesh).unwrap();
        }

        // all tiles have been updated, we can remove the requests from the list
        entities.clear()
    }
}




///
/// Wrapper for a Vec of Entites that need get a mesh and texture
///
pub struct UpdateQueue<'a>{
    entities: Vec<(Entity, &'a str)>,
}

impl<'a> std::ops::Deref for UpdateQueue<'a> {
    type Target = Vec<(Entity, &'a str)>;
    fn deref(&'a self) -> &Self::Target {
        &self.entities
    }
}

impl<'a> std::ops::DerefMut for UpdateQueue<'a> {
    fn deref_mut(&mut self) -> &'a mut Self::Target {
        &mut self.entities
    }
}

impl<'a> Default for UpdateQueue<'a> {
    fn default() -> Self {
        UpdateQueue {
            entities: Vec::<(Entity, &str)>::new(),
        }
    }
}
