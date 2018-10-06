use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, MaterialDefaults, Event, Mesh, ObjFormat, Projection, SpriteRenderData, VirtualKeyCode};

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

fn load_grid() -> Grid {
    let level_grid = Grid::load(Path::new(&format!(
        "{}/assets/levels/1.ron",
        env!("CARGO_MANIFEST_DIR")
    )));
    debug!("{:?}", level_grid);

    level_grid
}


fn initialize_level_grid(world: &mut World, grid_config: Grid) {
    let mut level_grid = LevelGrid::from_grid(grid_config, world);
    {
        let mut asset_manager = world.write_resource::<AssetManager<Mesh>>();
        let loader = world.read_resource::<Loader>();
        for x in 0..level_grid.grid().len() {
            for y in 0..level_grid.grid()[x].len() {

                let entity = level_grid.get(x, y);
                // always returns (0,0) so far

                let (wall_type, wall_direction) = level_grid.determine_sprite_for(x, y, world);
                //TODO use wall type
                match 0 {
                    0 => {
                        let material = world.read_resource::<MaterialDefaults>().0.clone();

                        let mesh = {
                            let mut mesh_storage = world.write_resource::<AssetStorage<Mesh>>();
                            asset_manager.load("meshes/wall.obj", ObjFormat, Default::default(), &mut mesh_storage, &loader).unwrap()
                        };
                        world.system_data::<SpriteRenderData>().meshes.insert(entity, mesh).unwrap();
                        world.system_data::<SpriteRenderData>().materials.insert(entity, material).unwrap();
                    }
                    //...
                    _ => {}
                }
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
