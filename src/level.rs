use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::ecs::Resources;
use amethyst::renderer::{
    Camera, Event, Material, MaterialDefaults, Mesh, MeshData, ObjFormat, PosTex, Projection,
    Shape, SpriteRenderData, Texture, TextureHandle, VirtualKeyCode, WindowMessages,
};

use entities::tile::*;
use game_data::CustomGameData;
use std::path::Path;

pub struct Level;

impl<'a, 'b> State<CustomGameData<'a, 'b>> for Level {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;
        world.register::<Tile>();

        load_assets(world);

        initialize_camera(world);
        let grid_config = load_grid(world);
        initialize_level_grid(world, grid_config);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: Event,
    ) -> Trans<CustomGameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else if is_key_down(&event, VirtualKeyCode::Tab) {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>> {
        data.data.update(&data.world, true);

        println!("LEVEL");
        Trans::None
    }
}

fn load_grid(world: &mut World) -> Grid {
    let level_grid = Grid::load(Path::new(&format!(
        "{}/assets/levels/1.ron",
        env!("CARGO_MANIFEST_DIR")
    )));
    debug!("{:?}", level_grid);

    level_grid
}

fn load_assets(world: &mut World) {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
    load_meshes(&loader, &mesh_storage);
}

fn load_meshes(loader: &Loader, mesh_storage: &AssetStorage<Mesh>) {
    let meshes = ["wall", "ground"];

    for mesh in meshes.iter() {
        loader.load(
            format!("meshes/{}.obj", mesh),
            ObjFormat,
            Default::default(),
            (), // we may wanna add a progress here
            &mesh_storage,
        );
    }
}

fn initialize_level_grid(world: &mut World, grid_config: Grid) {
    let mut level_grid = LevelGrid::from_grid(grid_config, world);
    level_grid.add_meshes(world);
    world.add_resource(level_grid);
}

/// initialize the camera.
fn initialize_camera(world: &mut World) {
    //todo remove all other camera entities
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
