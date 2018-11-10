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
use std::cmp::Reverse;

use assetloading::asset_loader::AssetManager;
use entities::tile::*;
use game_data::CustomGameData;
use std::time::Duration;
use systems::TileUpdateQueue;

use std::collections::BinaryHeap;

use std::path::Path;

pub struct Level;

impl Level {
    fn load_tile_pattern_config() -> Vec<([[Tile; 3]; 3], String)> {
        let result = Vec::<([[Tile; 3]; 3], String)>::load(Path::new(&format!(
            "{}/resources/tile_config.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded TilePattern successfully");
        result
    }

    fn load_grid() -> Grid {
        let level_grid = Grid::load(Path::new(&format!(
            "{}/assets/levels/1.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded Grid successfully");
        level_grid
    }

    fn initialize_level_grid(world: &mut World, grid: Grid) {
        let level_grid = LevelGrid::from_grid(grid, world);
        let max_x = level_grid.grid().len();
        let max_y = level_grid.grid()[0].len();

        world.add_resource(level_grid);

        let mut queue = world.write_resource::<TileUpdateQueue>();
        for x in 0..max_x {
            for y in 0..max_y {
                // write every coordinate in the update list to update every tile's mesh ans material
                queue.push((x as i32, y as i32));
            }
        }
    }

    fn load_initial_assets(world: &World) {
        let to_load = [
            "wall",
            "ground",
            "concealed",
            "single_edge",
            "single_edge_270",
        ];
        let mut mesh_manager = world.write_resource::<AssetManager<Mesh>>();
        let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
        let mut texture_manager = world.write_resource::<AssetManager<Texture>>();
        let mut texture_storage = world.write_resource::<AssetStorage<Texture>>();
        let loader = world.read_resource::<Loader>();

        for asset in to_load.iter() {
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
        mat.move_global(Vector3::new(-2., 6.0, 2.0));
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

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Level {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        debug!("Entering Level state");

        let world = data.world;
        world.register::<Tile>();
        world.register::<Light>();

        world.add_resource(BinaryHeap::<(Duration, Entity)>::new());

        //TODO refactor AssetLoader if this gets out of hand
        world.register::<AssetManager<Mesh>>();
        world.register::<AssetManager<Texture>>();

        let mesh_manager = AssetManager::<Mesh>::default();
        let texture_manager = AssetManager::<Texture>::default();
        world.add_resource(mesh_manager);
        world.add_resource(texture_manager);

        Level::load_initial_assets(world);

        let tile_pattern_config = Level::load_tile_pattern_config();
        world.add_resource(tile_pattern_config);

        let cam = Level::initialize_camera(world);
        Level::initialize_light(world, cam);
        let grid_config = Level::load_grid();
        Level::initialize_level_grid(world, grid_config);
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
                let mut heap = data
                    .world
                    .write_resource::<BinaryHeap<Reverse<(Duration, Entity)>>>();
                heap.push(Reverse((
                    data.world.read_resource::<Time>().absolute_time(),
                    entity,
                )));
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
