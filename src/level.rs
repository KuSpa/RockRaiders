use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::transform::{GlobalTransform, Parent, Transform};
use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Light, Material, MaterialDefaults, Mesh, ObjFormat, PngFormat, PointLight, Projection,
    Rgba, Texture, TextureMetadata, VirtualKeyCode,
};

use assetloading::asset_loader::AssetManager;
use entities::tile::*;
use game_data::CustomGameData;
use std::time::Duration;

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

    fn initialize_level_grid(world: &mut World, grid_config: Grid) {
        let level_grid = LevelGrid::from_grid(grid_config, world);

        {

            let dict = world.read_resource::<Vec<([[Tile; 3]; 3], String)>>();
            let mut mesh_manager = world.write_resource::<AssetManager<Mesh>>();
            let mut texture_manager = world.write_resource::<AssetManager<Texture>>();
            let loader = world.read_resource::<Loader>();
            for x in 0..level_grid.grid().len() {
                for y in 0..level_grid.grid()[x].len() {

                    let dict = world.read_resource::<Vec<([[Tile; 3]; 3], String)>>();
                    let (wall_type, wall_rotation) = level_grid.determine_sprite_for(x, y,&dict, &world.read_storage());

                    let entity = level_grid.get(x as i32, y as i32).unwrap();

                    let mut transform = Transform::default();
                    transform.set_position(Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: -(y as f32),
                    });

                    //add rotation to local transform
                    transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(wall_rotation as f32));

                    let mesh_path = format!("meshes/{}.obj", wall_type);
                    let texture_path = format!("textures/{}.png", wall_type);

                    let material = {
                        let mut texture_storage = world.write_resource::<AssetStorage<Texture>>();
                        let handle = texture_manager.get_asset_handle_or_load(
                            &texture_path,
                            PngFormat,
                            TextureMetadata::srgb(),
                            &mut texture_storage,
                            &loader,
                        );
                        Material {
                            albedo: handle,
                            ..world.read_resource::<MaterialDefaults>().0.clone()
                        }
                    };

                    let mesh = {
                        let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
                        mesh_manager.get_asset_handle_or_load(
                            &mesh_path,
                            ObjFormat,
                            Default::default(),
                            &mut mesh_storage,
                            &loader,
                        )
                    };

                    world.write_storage().insert(entity, mesh).unwrap();
                    world.write_storage().insert(entity, material).unwrap();
                    world.write_storage().insert(entity, transform).unwrap();
                    world
                        .write_storage()
                        .insert(entity, GlobalTransform::default())
                        .unwrap();
                }
            }
        }

        world.add_resource(level_grid);
    }

    /// initialize the camera.
    fn initialize_camera(world: &mut World) -> Entity {
        {
            let mut storage = world.write_storage::<Camera>();
            storage.clear();
        }
        let mut mat = Transform::default();
        mat.move_global(Vector3::new(0., 3.0, 0.0));
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

        let am = AssetManager::<Mesh>::default();
        world.add_resource(am);
        let am = AssetManager::<Texture>::default();
        world.add_resource(am);

        let tile_pattern_config = Level::load_tile_pattern_config();
        world.add_resource(tile_pattern_config);

        let cam = Level::initialize_camera(world);
        Level::initialize_light(world, cam);
        let grid_config = Level::load_grid();
        Level::initialize_level_grid(world, grid_config);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                debug!("Quitting");
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                debug!("Leaving Level State");
                return Trans::Pop;
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
