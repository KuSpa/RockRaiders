extern crate amethyst;
#[macro_use]
extern crate log;

mod entities;
mod rockraiders;
mod systems;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawFlat, PosTex};
use amethyst::{LogLevelFilter, LoggerConfig};

fn main() -> amethyst::Result<()> {
    let logger_config = LoggerConfig {
        use_colors: true,
        level_filter: LogLevelFilter::Info,
    };

    amethyst::start_logger(logger_config);
    use rockraiders::RockRaiders;

    let path = format!("{}/resources/display.ron", env!("CARGO_MANIFEST_DIR"));

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = format!("{}/", env!("CARGO_MANIFEST_DIR"));

    let input = InputBundle::<String, String>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(input)?
        .with(
            systems::CameraMovementSystem,
            "camera_movement_system",
            &["input_system"],
        ).with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?;
    let mut game = Application::new(assets_dir, RockRaiders, game_data)?;
    game.run();
    Ok(())
}
