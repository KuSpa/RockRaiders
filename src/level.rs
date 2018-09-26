use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, Material, MaterialDefaults, Mesh, MeshData, PngFormat, PosTex, Projection,
    Shape, SpriteRenderData, Texture, TextureHandle, VirtualKeyCode, WindowMessages,
};

use entities::tile::Grid;
use game_data::CustomGameData;
use std::path::Path;

pub struct Level;

impl<'a, 'b> State<CustomGameData<'a, 'b>> for Level {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;
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

fn initialize_level_grid(world: &mut World, grid_config: Grid) {



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
