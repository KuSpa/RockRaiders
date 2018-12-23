use amethyst::{
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Parent, ParentHierarchy, Transform},
    },
    ecs::prelude::{Builder, Component, Entity, NullStorage, World},
};

use rand::prelude::*;

use assetmanagement::util::{add_hover_handler, insert_into_asset_storages};
use entities::{RockRaider, Tile};
use eventhandling::Clickable;
use level::LevelGrid;
use util::amount_in;

use ncollide3d::shape::{Cuboid, Shape};

const MAX_RAIDERS: usize = 10;

/// A tag to indicate the entity as `Base`
/// The `Base` is the main building of each Level with which you start with and where you can spawn additional `RockRaider`
pub struct Base;

impl Base {
    pub fn spawn_rock_raider(&self, own_entity: Entity, world: &World) -> Entity {
        {
            let rr_storage = world.read_storage::<RockRaider>();
            if amount_in(&rr_storage) >= MAX_RAIDERS {
                panic!(
                    "Cannot spawn more Raiders. Limit of {} is already reached",
                    MAX_RAIDERS
                );
            }
        }

        let spawn_position = {
            let parent = world
                .read_storage::<Parent>()
                .get(own_entity)
                .unwrap()
                .entity;
            let tiles = world.read_storage::<Tile>();
            let level_grid = world.read_resource::<LevelGrid>();
            let transforms = world.read_storage::<Transform>();
            let possible_spawns = level_grid.walkable_neighbors(&parent, &tiles, &transforms);

            // when the spawns are empty, something went horribly wrong
            assert!(!possible_spawns.is_empty());

            let spawn_index = ((rand::thread_rng().gen::<f32>() * possible_spawns.len() as f32)
                .floor() as usize)
                // in case `thread_rng` returned exactly 1, we need to subtract 1 in the end
                .min(possible_spawns.len() - 1);
            let spawn_tile_position = transforms
                .get(possible_spawns[spawn_index])
                .unwrap()
                .translation();
            Point2::new(spawn_tile_position.x, spawn_tile_position.z)
        };

        let storages = world.system_data();
        let hover_storage = world.system_data();
        let click_storage = world.system_data();
        let entities = world.entities();
        RockRaider::instantiate(
            &entities,
            spawn_position,
            storages,
            hover_storage,
            click_storage,
        )
    }

    /// Create a new Base. The given entity has to have a `Tile::Ground` Component, which then is used as Parent to determine the Position
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

        let base = Base::default();

        let result = world
            .create_entity()
            .with(Transform::default()) // since we will be a child, this should translate us to the perfect spot ;)
            .with(GlobalTransform::default())
            .with(base)
            .with(Parent { entity: *entity })
            .build();

        {
            let mut storages = world.system_data();
            add_hover_handler(
                result,
                Base::asset_name(),
                Base::bounding_box(),
                &mut storages,
            );
        }

        {
            let mut storages = world.system_data();
            insert_into_asset_storages(result, Base::asset_name(), &mut storages);
        }

        //Build Click Handler
        {
            use eventhandling::*;
            let tmp: Box<dyn Clickable> = Box::new(Base);
            world
                .write_storage::<Box<dyn Clickable>>()
                .insert(result, tmp)
                .unwrap();
        }
    }

    fn asset_name() -> &'static str {
        "buildings/base"
    }

    fn bounding_box() -> Box<dyn Shape<f32>> {
        Box::new(Cuboid::new(Vector3::new(0.33, 0.33, 0.38)))
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

impl Clickable for Base {
    /// This method is called, whenever the mouse hovers the entity of this component. It only is triggered on the nearest entity, that has a `Hoverable` Comonent as well.
    fn on_click(&self, entity: Entity, world: &World) {
        self.spawn_rock_raider(entity, world);
    }
}
