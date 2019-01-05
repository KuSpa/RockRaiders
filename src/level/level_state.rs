use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        nalgebra::Vector3,
        timing::Time,
        transform::{GlobalTransform, Parent, Transform},
    },
    ecs::{Entity, WriteStorage},
    input::{is_close_requested, is_key_down, InputHandler},
    prelude::*,
    renderer::{
        ActiveCamera, Camera, Light, Mesh, MouseButton, ObjFormat, PngFormat, PointLight, Rgba,
        ScreenDimensions, Texture, TextureMetadata, VirtualKeyCode,
    },
    shrev::EventChannel,
    ui::*,
};

use assetmanagement::AssetManager;
use entities::{buildings::Base, RockRaider, Tile};
use eventhandling::{ClickHandlerComponent, GameEvent, HoverEvent, HoverHandlerComponent, Hovered};
use level::{LevelGrid, TileGrid};
use systems::{Oxygen, OxygenBar, Path, RevealQueue};
use ui::UiMap;
use util::add_resource_soft;
use GameScene;

use std::{
    cmp::Reverse,
    ops::{Deref, DerefMut},
    path::Path as OSPath,
};

pub struct SelectedRockRaider(pub Entity);

impl Deref for SelectedRockRaider {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SelectedRockRaider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The `State` that is active, when a level runs
pub struct LevelState {
    /// pretty self explanatory
    pub mouse_button_was_down: bool,
}

/// This is a Map referencing from a 3x3 Tile matrix to a String.
/// The 3x3 describes a pattern (for example 9 times a `Tile::Wall`) that can occur in the cave's layout and maps to the asset's name that is representing it:
///
/// example:
/// ```
/// (
///  [[Ground, Ground, Ground],
///   [Wall,   Wall,   Wall],
///   [Wall,   Wall,   Wall]]
///  , "straight_wall"
/// ```
///
pub type TilePatternMap = Vec<([[Tile; 3]; 3], String)>;

impl LevelState {
    /// Loads the `TilePatternMap` from disk.
    fn load_tile_pattern_config() -> TilePatternMap {
        let result = TilePatternMap::load(OSPath::new(&format!(
            "{}/resources/tile_config.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded TilePattern successfully");
        result
    }

    /// Loads the cave's model from disk.
    fn load_tile_grid() -> TileGrid {
        let tile_grid = TileGrid::load(OSPath::new(&format!(
            "{}/assets/levels/1.ron",
            env!("CARGO_MANIFEST_DIR")
        )));

        debug!("Loaded Grid successfully");
        tile_grid
    }

    /// Converts the cave's model into a `LevelGrid` and adds it to the world.
    fn initialize_level_grid(world: &mut World, tile_grid: TileGrid) {
        let level_grid = LevelGrid::from_grid(tile_grid, world);
        let max_x = level_grid.x_len();
        let max_y = level_grid.y_len();
        {
            let tiles = world.read_storage::<Tile>();
            let mut transforms = world.write_storage::<Transform>();
            let dict = world.read_resource::<TilePatternMap>();
            let mut tex_storages = world.system_data();
            let mut mesh_storages = world.system_data();
            let mut hover_storage = world.system_data::<WriteStorage<HoverHandlerComponent>>();
            let mut click_storage = world.system_data::<WriteStorage<ClickHandlerComponent>>();
            let mut hovered = world.write_resource::<Hovered>();
            let mut hover_channel = world.write_resource::<EventChannel<HoverEvent>>();
            let loader = world.read_resource();

            for x in 0..max_x {
                for y in 0..max_y {
                    level_grid.update_tile(
                        x as i32,
                        y as i32,
                        &dict,
                        &mut transforms,
                        &tiles,
                        &mut tex_storages,
                        &mut mesh_storages,
                        &mut hovered,
                        &loader,
                        &mut hover_channel,
                        &mut hover_storage,
                        &mut click_storage,
                    );
                }
            }
        }
        world.add_resource(level_grid);
    }

    /// Loads all assets that will presumably be used in the level into memory and `AssetManager`.
    fn load_initial_assets(world: &World) {
        let mut mesh_manager = world.write_resource::<MeshManager>();
        let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
        let mut texture_manager = world.write_resource::<TextureManager>();
        let mut texture_storage = world.write_resource::<AssetStorage<Texture>>();
        let loader = world.read_resource();

        for (_, asset) in world.read_resource::<TilePatternMap>().iter() {
            debug!("loading asset: {}", asset);
            mesh_manager.get_handle_or_load(asset, &loader, &mut mesh_storage);
            texture_manager.get_handle_or_load(asset, &loader, &mut texture_storage);
        }
    }

    /// Creates an camera entity and sets it as `Activecamera`.
    fn initialize_camera(world: &mut World) -> Entity {
        {
            let mut storage = world.write_storage::<Camera>();
            storage.clear();
        }
        let mut mat = Transform::default();
        mat.move_global(Vector3::new(-1., 6.0, 7.0));
        mat.yaw_global(-std::f32::consts::FRAC_PI_4);
        mat.pitch_local(-std::f32::consts::FRAC_PI_4);

        let (screen_w, screen_h) = {
            let dims = world.read_resource::<ScreenDimensions>();
            (dims.width(), dims.height())
        };

        let entity = world
            .create_entity()
            .with(Camera::standard_3d(screen_w, screen_h))
            .with(mat)
            .with(GlobalTransform::default())
            .build();

        world.add_resource(ActiveCamera { entity });
        entity
    }

    /// Adds a light as child to the given entity. Assumes that the given Entity is the `ActiveCamera` of the Level.
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

    /// Creates a `Base` for the Level.
    fn initialize_base(world: &mut World) {
        let entity = world.read_resource::<LevelGrid>().get(2, 0).unwrap();
        {
            if let Some(ref mut queue) = *world.write_resource::<Option<RevealQueue>>() {
                queue.push(Reverse((
                    world.read_resource::<Time>().absolute_time(),
                    entity,
                )));
            }
            Base::build(&entity, world);
        }
    }

    fn scene() -> GameScene {
        GameScene::Level
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, GameEvent> for LevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.register::<Tile>();
        world.register::<Light>();
        world.register::<Base>();
        world.register::<HoverHandlerComponent>();
        world.register::<ClickHandlerComponent>();
        world.register::<RockRaider>();
        world.register::<Path>();

        let mesh_manager = AssetManager::<Mesh>::default();
        let texture_manager = AssetManager::<Texture>::default();
        let tile_pattern_config = LevelState::load_tile_pattern_config();
        let oxygen = Oxygen::new(100.);
        let tile_grid = LevelState::load_tile_grid();
        let map = UiMap::from(tile_grid.clone(), world);

        world.exec(|mut creator: UiCreator| creator.create("ui/oxygen_bar/prefab.ron", ()));

        world.add_resource(Some(RevealQueue::new()));
        world.add_resource(Some(oxygen));
        world.add_resource::<Hovered>(Hovered::default());
        world.add_resource::<Option<OxygenBar>>(None);
        world.add_resource::<Option<SelectedRockRaider>>(None);
        world.add_resource(Some(map));

        add_resource_soft(world, mesh_manager);
        add_resource_soft(world, texture_manager);
        add_resource_soft(world, tile_pattern_config);

        LevelState::load_initial_assets(world);
        let cam = LevelState::initialize_camera(world);
        LevelState::initialize_light(world, cam);
        LevelState::initialize_level_grid(world, tile_grid);

        *world.write_resource() = LevelState::scene();
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = LevelState::scene();
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: GameEvent,
    ) -> Trans<GameData<'a, 'b>, GameEvent> {
        let world = data.world;
        match &event {
            // Dispatch the incoming event
            GameEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    debug!("Quitting");
                    return Trans::Quit;
                } else if is_key_down(&event, VirtualKeyCode::Tab) {
                    debug!("Leaving Level State");
                    return Trans::Pop;
                } else if is_key_down(&event, VirtualKeyCode::Space) {
                    do_test_method(world);

                    return Trans::None;
                }
            }
            GameEvent::Hover(event) => {
                // the following code may be a bit unintuitive:
                // # remove handler
                // # execute handler
                // # add handler again
                // This is required, because the handler itself may fetch the hoverhandler storage on execution, what would lead to a new borrow, while this method still borrows the storage to execute the handler.
                // To bypass this, we remove the handler for the time of execution, so that no resource of the world is borrowed and there are no possible `Invalid Borrow` clashes from this side of the code.
                let mut hover_handler = world
                    .write_storage::<HoverHandlerComponent>()
                    .remove(event.target)
                    .unwrap();
                if event.start {
                    // hover started
                    hover_handler.on_hover_start(event.target, world);
                } else {
                    // hover ended
                    hover_handler.on_hover_stop(event.target, world);
                }
                world
                    .write_storage::<HoverHandlerComponent>()
                    .insert(event.target, hover_handler)
                    .unwrap();
            }
            _ => (),
        }

        let mouse_button = world
            .read_resource::<InputHandler<String, String>>()
            .mouse_button_is_down(MouseButton::Left);

        if !self.mouse_button_was_down && mouse_button {
            if let Some(entity) = **(world.read_resource::<Hovered>()) {
                // see hover event dispatching
                let opt_handler = world
                    .write_storage::<ClickHandlerComponent>()
                    .remove(entity);
                opt_handler.map(|handler| {
                    handler.on_click(entity, world);
                    world
                        .write_storage::<ClickHandlerComponent>()
                        .insert(entity, handler)
                });
            }
        }
        self.mouse_button_was_down = mouse_button;

        // reset selection on right click
        if world
            .read_resource::<InputHandler<String, String>>()
            .mouse_button_is_down(MouseButton::Right)
        {
            *world.write_resource::<Option<SelectedRockRaider>>() = None;
        }
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;
        *world.write_resource() = GameScene::default();
        *world.write_resource::<Option<SelectedRockRaider>>() = None;
        *world.write_resource::<Hovered>() = Hovered::default();
        *world.write_resource::<Option<UiMap>>() = None;
        *world.write_resource::<Option<OxygenBar>>() = None;
        *world.write_resource::<Option<Oxygen>>() = None;
        *world.write_resource::<Option<RevealQueue>>() = None;
        *world.write_resource::<LevelGrid>() = LevelGrid::default(); //Option?

        world.maintain();
        world.delete_all();
    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, GameEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn do_test_method(world: &mut World) {
    LevelState::initialize_base(world);
}
