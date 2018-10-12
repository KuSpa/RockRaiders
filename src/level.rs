use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, MaterialDefaults, Mesh, ObjFormat, Projection, SpriteRenderData, VirtualKeyCode,
};

use assetloading::asset_loader::AssetManager;
use entities::tile::*;

use game_data::CustomGameData;
use std::path::Path;

pub struct Level;

impl<'a, 'b> State<CustomGameData<'a, 'b>> for Level {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        debug!("Entering Level state");

        let world = data.world;
        world.register::<Tile>();

        //TODO Add for all Types
        world.register::<AssetManager<Mesh>>();

        let am = AssetManager::<Mesh>::default();
        world.add_resource(am);

        let tile_pattern_config = load_tile_pattern_config();
        warn!("{:?}", tile_pattern_config);
        world.add_resource(tile_pattern_config);

        initialize_camera(world);
        let grid_config = load_grid();
        initialize_level_grid(world, grid_config);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: Event,
    ) -> Trans<CustomGameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            debug!("Quitting");
            Trans::Quit
        } else if is_key_down(&event, VirtualKeyCode::Tab) {
            debug!("Leaving Level State");
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>> {
        data.data.update(&data.world, true);
        Trans::None
    }
}

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
        let mut asset_manager = world.write_resource::<AssetManager<Mesh>>();
        let loader = world.read_resource::<Loader>();
        for x in 0..level_grid.grid().len() {
            for y in 0..level_grid.grid()[x].len() {
                let (wall_type, wall_rotation) = level_grid.determine_sprite_for(x, y, world);

                let entity = level_grid.get(x, y);

                let mut transform = Transform::default();
                transform.set_position(Vector3 {
                    x: x as f32,
                    y: 0.0,
                    z: -(y as f32),
                });

                //add rotation to local transform
                transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(wall_rotation as f32));

                let mesh_path = format!("meshes/{}", wall_type);

                // TODO refactor - use texture path and load the real texture ;)
                let material = world.read_resource::<MaterialDefaults>().0.clone();

                let mesh = {
                    let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
                    asset_manager.get_asset_handle_or_load(
                        &mesh_path,
                        ObjFormat,
                        Default::default(),
                        &mut mesh_storage,
                        &loader,
                    )
                };

                world
                    .system_data::<SpriteRenderData>()
                    .meshes
                    .insert(entity, mesh)
                    .unwrap();
                world
                    .system_data::<SpriteRenderData>()
                    .materials
                    .insert(entity, material)
                    .unwrap();
                world
                    .write_storage::<Transform>()
                    .insert(entity, transform)
                    .unwrap();
                world
                    .write_storage::<GlobalTransform>()
                    .insert(entity, GlobalTransform::default())
                    .unwrap();
            }
        }
    }

    world.add_resource(level_grid);
}

/// initialize the camera.
fn initialize_camera(world: &mut World) {
    //Todo remove all other camera entities
    let mut mat = Transform::default();
    mat.move_global(Vector3::new(0., 3.0, 0.0));
    mat.yaw_global(Deg(-45.0));
    mat.pitch_local(Deg(-45.0));

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
        .with(mat)
        .with(GlobalTransform::default())
        .build();
}
