use amethyst::core::transform::{GlobalTransform, Parent, ParentHierarchy, Transform};
use amethyst::ecs::prelude::{Component, Entity, NullStorage};
use amethyst::prelude::*;
use amethyst::Error;
use amethyst::Result;

use assetmanagement::util::insert_into_storages;
use entities::Tile;

pub struct Base;

impl Base {
    pub fn try_instantiating(entity: &Entity, world: &mut World) -> Result<Entity> {
        {
            // if the entity has children, they have to be buildings so far.
            // If we will ever have other children than buildings, we will have to test for every children, if it is a building
            let hierarchy = world.read_resource::<ParentHierarchy>();
            if !hierarchy.children(*entity).is_empty() {
                // TODO Custom error types for better failure dispatch
                // ERROR another building occupies this tile
                return Err(Error::Application);
            }
        }

        // if the tile is a revealed ground tile, we continue, otherwise we return
        match world.read_storage::<Tile>().get(*entity) {
            Some(Tile::Ground { .. }) => (),
            // TODO Custom error types for better failure dispatch
            // ERROR cannot build on Walls
            _ => return Err(Error::Application),
        }
        Ok(Base::build(entity, world))
    }

    fn build(entity: &Entity, world: &mut World) -> Entity {
        let base = Base;

        let result = world
            .create_entity()
            .with(Transform::default()) // since we will be a child, this should translate us to the perfect spot ;)
            .with(GlobalTransform::default())
            .with(base)
            .with(Parent { entity: *entity })
            .build();

        let mut storages = world.system_data();
        insert_into_storages(result, Base::asset_name(), &mut storages);

        result
    }

    fn asset_name() -> &'static str {
        "buildings/base"
    }
}

impl Component for Base {
    type Storage = NullStorage<Base>;
}

impl Default for Base {
    fn default() -> Self {
        Base
    }
}
