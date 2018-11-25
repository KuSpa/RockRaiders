use amethyst::assets::{AssetStorage, Loader};

use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::transform::{GlobalTransform, Parent, Transform};

use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Light, Mesh, ObjFormat, PngFormat, PointLight, Projection, Rgba, Texture,
    TextureMetadata, VirtualKeyCode,
};

use assetmanagement::{util::insert_from_world, AssetManager};
use entities::buildings::Base;
use entities::Tile;
use game_data::CustomGameData;
use level::LevelGrid;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Duration;

use std::path::Path;

pub struct LevelState;

impl LevelState {
    fn load_tile_pattern_config() -> Vec<([[Tile; 3]; 3], String)> {
        let result = Vec::<([[Tile; 3]; 3], String)>::load(Path::new(&format!(
            "{}/resources/tile_config.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded TilePattern successfully");
        result
    }

    fn load_grid() -> Vec<Vec<Tile>> {
        let grid = Vec::<Vec<Tile>>::load(Path::new(&format!(
            "{}/assets/levels/1.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded Grid successfully");
        grid
    }

    fn initialize_level_grid(
        world: &mut World,
        grid: Vec<Vec<Tile>>,
        dict: &Vec<([[Tile; 3]; 3], String)>,
    ) {
        let level_grid = LevelGrid::from_grid(grid, world);
        let max_x = level_grid.grid().len();
        let max_y = level_grid.grid()[0].len();
        {
            let mut tiles = world.write_storage::<Tile>();
            let mut transforms = world.write_storage::<Transform>();

            for x in 0..max_x {
                for y in 0..max_y {
                    let (classifier, rotation) =
                        level_grid.determine_sprite_for(x as i32, y as i32, dict, &mut tiles);
                    let entity = level_grid.get(x as i32, y as i32).unwrap();
                    level_grid.adjust_transform(x as i32, y as i32, rotation, &mut transforms);
                    insert_from_world(entity, classifier, world);
                }
            }
        }
        world.add_resource(level_grid);
    }

    fn load_initial_assets(world: &World) {
        let mut mesh_manager = world.write_resource::<AssetManager<Mesh>>();
        let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
        let mut texture_manager = world.write_resource::<AssetManager<Texture>>();
        let mut texture_storage = world.write_resource::<AssetStorage<Texture>>();
        let loader = world.read_resource::<Loader>();

        for (_, asset) in world
            .read_resource::<Vec<([[Tile; 3]; 3], String)>>()
            .iter()
        {
            warn!("loading asset: {}", asset);
            mesh_manager.get_asset_handle_or_load(
                asset,
                ObjFormat,
                Default::default(),
                &mut mesh_storage,
                &loader,
            );
            texture_manager.get_asset_handle_or_load(
                asset,
                PngFormat,
                TextureMetadata::srgb(),
                &mut texture_storage,
                &loader,
            );
        }
    }

    /// initialize the camera.
    fn initialize_camera(world: &mut World) -> Entity {
        {
            let mut storage = world.write_storage::<Camera>();
            storage.clear();
        }
        let mut mat = Transform::default();
        mat.move_global(Vector3::new(-2., 6.0, 4.0));
        mat.yaw_global(Deg(-45.0));
        mat.pitch_local(Deg(-45.0));

        world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
            .with(mat)
            .with(GlobalTransform::default())
            .build()
    }

    fn initialize_light(world: &mut World, parent: Entity) {
        let light = PointLight {
            color: Rgba::white(),
            intensity: 25.,
            radius: 0.1,
            smoothness: 0.5,
        };
        world
            .create_entity()
            .with(Light::from(light))
            .with(GlobalTransform::default())
            .with(Transform::default())
            .with(Parent { entity: parent })
            .build();
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LevelState {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        debug!("Entering Level state");

        let world = data.world;
        world.register::<Tile>();
        world.register::<Light>();
        world.register::<Base>();

        world.add_resource(BinaryHeap::<(Duration, Entity)>::new());

        world.register::<AssetManager<Mesh>>();
        world.register::<AssetManager<Texture>>();

        let mesh_manager = AssetManager::<Mesh>::default();
        let texture_manager = AssetManager::<Texture>::default();
        world.add_resource(mesh_manager);
        world.add_resource(texture_manager);

        let tile_pattern_config = LevelState::load_tile_pattern_config();

        LevelState::load_initial_assets(world);

        let cam = LevelState::initialize_camera(world);
        LevelState::initialize_light(world, cam);
        let grid_definition = LevelState::load_grid();
        LevelState::initialize_level_grid(world, grid_definition, &tile_pattern_config);

        world.add_resource(tile_pattern_config);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                debug!("Quitting");
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                debug!("Leaving Level State");
                return Trans::Pop;
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                debug!("Start revealing");
                let entity = data.world.read_resource::<LevelGrid>().get(2, 0).unwrap();
                {
                    let mut heap = data
                        .world
                        .write_resource::<BinaryHeap<Reverse<(Duration, Entity)>>>();
                    heap.push(Reverse((
                        data.world.read_resource::<Time>().absolute_time(),
                        entity,
                    )));
                }
                Base::try_instantiating(&entity, data.world).unwrap();
                return Trans::None;
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}