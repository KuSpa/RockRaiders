use collision::primitive::{Cuboid, Primitive3};
use rand::prelude::*;

use amethyst::core::cgmath::Point2;
use amethyst::core::transform::{GlobalTransform, Parent, ParentHierarchy, Transform};
use amethyst::ecs::prelude::{Builder, Component, Entity, NullStorage, World};

use assetmanagement::util::{add_hover_handler, insert_into_asset_storages};
use entities::{RockRaider, Tile};
use level::LevelGrid;
use util::amount_in;

const MAX_RAIDERS: usize = 10;

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
                .translation;
            Point2 {
                x: spawn_tile_position.x,
                y: spawn_tile_position.z,
            }
        };

        let mut storages = world.system_data();
        let entities = world.entities();
        RockRaider::instantiate(&entities, spawn_position, &mut storages)
    }

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

    fn bounding_box() -> Primitive3<f32> {
        Primitive3::Cuboid(Cuboid::<f32>::new(1., 1., 1.))
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
