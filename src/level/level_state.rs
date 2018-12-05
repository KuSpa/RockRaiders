use amethyst::assets::{AssetStorage, Loader};

use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::transform::{GlobalTransform, Parent, Transform};

use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down, InputHandler};
use amethyst::prelude::*;
use amethyst::renderer::{
    ActiveCamera, Camera, Light, Mesh, MouseButton, ObjFormat, PngFormat, PointLight, Projection,
    Rgba, Texture, TextureMetadata, VirtualKeyCode,
};

use eventhandling::Clickable;
use systems::Oxygen;

use assetmanagement::AssetManager;
use entities::buildings::Base;
use entities::RockRaider;
use entities::Tile;
use game_data::CustomGameData;
use level::LevelGrid;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Duration;
use systems::HoverHandler;
use systems::Hovered;
use systems::Path;

use std::path::Path as OSPath;

pub struct LevelState {
    pub mouse_button_was_down: bool,
}

pub type TilePatternMap = Vec<([[Tile; 3]; 3], String)>;

impl LevelState {
    fn load_tile_pattern_config() -> TilePatternMap {
        let result = TilePatternMap::load(OSPath::new(&format!(
            "{}/resources/tile_config.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded TilePattern successfully");
        result
    }

    fn load_tile_grid() -> Vec<Vec<Tile>> {
        let tile_grid = Vec::<Vec<Tile>>::load(OSPath::new(&format!(
            "{}/assets/levels/1.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded Grid successfully");
        tile_grid
    }

    fn initialize_level_grid(world: &mut World, tile_grid: Vec<Vec<Tile>>) {
        let level_grid = LevelGrid::from_grid(tile_grid, world);
        let max_x = level_grid.x_len();
        let max_y = level_grid.y_len();
        {
            let tiles = world.read_storage::<Tile>();
            let mut transforms = world.write_storage::<Transform>();
            let dict = world.read_resource::<TilePatternMap>();
            let mut storages = world.system_data();

            for x in 0..max_x {
                for y in 0..max_y {
                    level_grid.update_tile(
                        x as i32,
                        y as i32,
                        &dict,
                        &mut transforms,
                        &tiles,
                        &mut storages,
                    );
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

        for (_, asset) in world.read_resource::<TilePatternMap>().iter() {
            debug!("loading asset: {}", asset);
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
        mat.move_global(Vector3::new(-1., 6.0, 7.0));
        mat.yaw_global(Deg(-45.0));
        mat.pitch_local(Deg(-45.0));

        let entity = world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
            .with(mat)
            .with(GlobalTransform::default())
            .build();

        // not sure about this line tho
        world.add_resource(ActiveCamera { entity });
        entity
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

    fn initialize_base(world: &mut World) {
        let entity = world.read_resource::<LevelGrid>().get(2, 0).unwrap();
        {
            let mut ground_reveal_queue =
                world.write_resource::<BinaryHeap<Reverse<(Duration, Entity)>>>();
            ground_reveal_queue.push(Reverse((
                world.read_resource::<Time>().absolute_time(),
                entity,
            )));
        }
        Base::build(&entity, world);
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LevelState {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;

        world.register::<Tile>();
        world.register::<Light>();
        world.register::<Base>();
        world.register::<HoverHandler>();
        world.register::<Box<dyn Clickable>>();

        world.add_resource(BinaryHeap::<(Duration, Entity)>::new());
        world.add_resource::<Option<Hovered>>(None);

        world.register::<RockRaider>();
        world.register::<AssetManager<Mesh>>();
        world.register::<AssetManager<Texture>>();
        world.register::<Path>();

        let mesh_manager = AssetManager::<Mesh>::default();
        let texture_manager = AssetManager::<Texture>::default();
        let tile_pattern_config = LevelState::load_tile_pattern_config();

        let oxygen = Oxygen {
            remaining_oxygen: 100.,
        };

        world.add_resource(oxygen);
        world.add_resource(mesh_manager);
        world.add_resource(texture_manager);
        world.add_resource(tile_pattern_config);
        world.add_resource(BinaryHeap::<(Duration, Entity)>::new());

        LevelState::load_initial_assets(world);

        let cam = LevelState::initialize_camera(world);
        LevelState::initialize_light(world, cam);

        LevelState::initialize_level_grid(world, LevelState::load_tile_grid());
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
                do_test_method(data);
                return Trans::None;
            }
        }

        let mouse_button = data
            .world
            .read_resource::<InputHandler<String, String>>()
            .mouse_button_is_down(MouseButton::Left);

        if !self.mouse_button_was_down && mouse_button {
            if let Some(hovered) = &*data.world.read_resource::<Option<Hovered>>() {
                let entity = hovered.entity;
                data.world
                    .read_storage::<Box<dyn Clickable>>()
                    .get(entity)
                    .map(|handler| handler.on_click(&entity, data.world));
            }
        }

        self.mouse_button_was_down = mouse_button;

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

fn do_test_method(data: StateData<CustomGameData>) {
    let mut world = data.world;
    LevelState::initialize_base(world);
}
