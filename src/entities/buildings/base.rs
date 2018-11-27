use amethyst::core::transform::{GlobalTransform, Parent, ParentHierarchy, Transform};
use amethyst::ecs::prelude::{Component, Entity, NullStorage};
use amethyst::prelude::*;

use assetmanagement::util::insert_into_asset_storages;
use entities::Tile;

pub struct Base;

impl Base {
    pub fn build(entity: &Entity, world: &mut World) {
        {
            // if the entity has children, they have to be buildings so far.
            // If we will ever have other children than buildings, we will have to test for every children, if it is a building
            let hierarchy = world.read_resource::<ParentHierarchy>();
            if !hierarchy.children(*entity).is_empty() {
                panic!("ERROR another building occupies this tile");
            }
        }

        // if the tile is a revealed ground tile, we continue, otherwise we return
        match world.read_storage::<Tile>().get(*entity) {
            Some(Tile::Ground { .. }) => (),

            _ => panic!("ERROR cannot build on Walls"),
        }

        let base = Base;

        let result = world
            .create_entity()
            .with(Transform::default()) // since we will be a child, this should translate us to the perfect spot ;)
            .with(GlobalTransform::default())
            .with(base)
            .with(Parent { entity: *entity })
            .build();

        let mut storages = world.system_data();
        insert_into_asset_storages(result, Base::asset_name(), &mut storages);
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
