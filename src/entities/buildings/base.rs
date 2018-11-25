use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::{GlobalTransform, Parent, ParentHierarchy, Transform};
use amethyst::ecs::prelude::{Component, Entity, NullStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureMetadata,
};
use amethyst::Error;
use amethyst::Result;

use assetloading::asset_loader::AssetManager;
use entities::Tile;
use util;

pub struct Base {
    // will get some members in the future i guess?
}

impl Base {
    pub fn try_instantiating(
        entity: &Entity,
        world: &mut World,
        //hierarchy: &ParentHierarchy,
    ) -> Result<Entity> {
        // if the entity has children (in context of the transform), they have to be buildings so far.
        // If we will ever have other children than buildings, we will have to test for every children, if it is a building
        //if !hierarchy.children(*entity).is_empty() {
        //    // TODO Custom error types for better failure dispatch
        //    return Err(Error::Application);
        //}

        // if the tile is a revealed ground tile, we continue, otherwise we return
        match world.read_storage::<Tile>().get(*entity) {
            Some(Tile::Ground { .. }) => (),
            // TODO Custom error types for better failure dispatch
            _ => return Err(Error::Application),
        }
        Base::build(entity, world)
    }

    fn build(entity: &Entity, world: &mut World) -> Result<Entity> {
        let base = Base {};

        let result = world
            .create_entity()
            .with(Transform::default()) // since we will be a child, this should translate us to the perfect spot ;)
            .with(GlobalTransform::default())
            .with(base)
            .with(Parent { entity: *entity })
            .build();

        let mut mesh_handles = world.write_storage::<MeshHandle>();
        let mut mat_storage = world.write_storage::<Material>();
        let mut mesh_manager = world.write_resource::<AssetManager<Mesh>>();
        let mut tex_manager = world.write_resource::<AssetManager<Texture>>();
        let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
        let mut tex_storage = world.write_resource::<AssetStorage<Texture>>();
        let mut loader = world.write_resource::<Loader>();
        let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

        util::insert_into_storages(
            *entity,
            Base::asset_name(),
            &loader,
            &mut mesh_manager,
            &mut mesh_handles,
            &mut mesh_storage,
            &mut tex_manager,
            &mut mat_storage,
            &mut tex_storage,
            default_mat,
        );

        Ok(result)
    }

    fn asset_name() -> &'static str {
        "buildings/base"
    }
}

impl Component for Base {
    // Maybe use a HashMaspStorage? it feels bad to have a Vec with a maximum of one Element
    // So far its just a flag
    type Storage = NullStorage<Base>;
}

impl Default for Base {
    fn default() -> Self {
        Base {}
    }
}
