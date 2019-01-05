use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Parent, ParentHierarchy, Transform},
    },
    ecs::prelude::{Builder, Component, Entity, NullStorage, World},
    renderer::{PngFormat, Texture, TextureMetadata},
};

use rand::prelude::*;

use assetmanagement::{util::attach_assets, MeshManager, TextureManager};
use entities::{RockRaider, Tile};
use eventhandling::{ClickHandlerComponent, Clickable, HoverHandlerComponent, SimpleHoverHandler};
use level::LevelGrid;
use util::amount_in;

use ncollide3d::shape::Cuboid;

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

        let mut rr_storages = world.system_data();
        let mut texture_storages = world.system_data();
        let mut mesh_storages = world.system_data();
        let hover_storage = world.system_data();
        let click_storage = world.system_data();
        let entities = world.entities();
        let loader = world.read_resource();
        RockRaider::instantiate(
            &entities,
            spawn_position,
            &mut rr_storages,
            &mut texture_storages,
            &mut mesh_storages,
            &loader,
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
            let mut tex_storages = world.system_data();
            let mut mesh_storages = world.system_data();
            let loader = world.read_resource();
            attach_assets(
                result,
                Base::asset_name(),
                &loader,
                &mut tex_storages,
                &mut mesh_storages,
            );
        }

        {
            // add a HoverHandler to the Entity
            let loader = world.read_resource::<Loader>();
            let mut tex_manager = world.write_resource();
            let mut tex_storage = world.write_resource();
            let handler = Base::new_hover_handler(&mut tex_manager, &loader, &mut tex_storage);
            world.write_storage().insert(result, handler).unwrap();
        }

        let mut click_storage = world.write_storage::<ClickHandlerComponent>();
        Base.attach_click_handler(result, &mut click_storage);
    }

    fn asset_name() -> &'static str {
        "buildings/base"
    }

    pub fn new_hover_handler(
        tex_manager: &mut TextureManager,
        loader: &Loader,
        mut tex_storage: &mut AssetStorage<Texture>,
    ) -> HoverHandlerComponent {
        let hover_mat = tex_manager.get_handle_or_load(
            format!(Self::asset_name(), "_hover").as_str(),
            &loader,
            &mut tex_storage,
        );

        let bounding_box = Cuboid::new(Vector3::new(0.33, 0.33, 0.39));
        Box::new(SimpleHoverHandler::new(bounding_box, hover_mat))
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
    fn on_click(&self, entity: Entity, world: &World) {
        self.spawn_rock_raider(entity, world);
    }

    fn new_click_handler(&self) -> ClickHandlerComponent {
        Box::new(Base) as ClickHandlerComponent
    }
}
