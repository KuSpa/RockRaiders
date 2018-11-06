extern crate amethyst;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod assetloading;
mod entities;
mod game_data;
mod level;
mod rockraiders;
mod systems;
mod util;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use entities::tile::Tile;
use std::path::Path;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use game_data::CustomGameDataBuilder;
    use rockraiders::RockRaiders;

    let path = format!("{}/resources/display.ron", env!("CARGO_MANIFEST_DIR"));

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

    let input = InputBundle::<String, String>::new();
    let dict = Vec::<([[Tile; 3]; 3], String)>::load(Path::new(&format!(
        "{}/resources/tile_config.ron",
        env!("CARGO_MANIFEST_DIR")
    )));

    let game_data = CustomGameDataBuilder::default()
        .with_core_bundle(input)?
        .with_core_bundle(TransformBundle::new())?
        .with_running(systems::CameraMovementSystem, "camera_movement_system", &[])
        .with_running(systems::TileUpdateSystem{dict}, "tile_update_system", &[])
        //.with_running(systems::GroundRevealSystem, "ground_reveal_system", &[])
        .with_basic_renderer(path, DrawShaded::<PosNormTex>::new())?;
    let mut game = Application::new(assets_dir, RockRaiders, game_data)?;
    game.run();
    Ok(())
}
