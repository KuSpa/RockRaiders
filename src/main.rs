extern crate amethyst;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ncollide3d;
extern crate pathfinding;
extern crate rand;

mod assetmanagement;
mod entities;
mod eventhandling;
mod level;
mod rockraiders;
mod systems;
mod util;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    use rockraiders::MainState;

    let path = format!("{}/resources/display.ron", env!("CARGO_MANIFEST_DIR"));

    let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

    let input = InputBundle::<String, String>::new();

    let config = DisplayConfig::load(path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::new()
        .with_bundle(input)?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::MouseRaySystem.pausable(GameScene::Level),
              "mouse_ray_system", &[])
        .with(
            systems::MovementSystem.pausable(GameScene::Level),
            "movement_system",
            &["transform_system"],
        )
        .with(
            systems::CameraMovementSystem.pausable(GameScene::Level),
            "camera_movement_system",
            &[],
        )
        .with(
            systems::GroundRevealSystem.pausable(GameScene::Level),
            "ground_reveal_system",
            &["transform_system"],
        )
        .with(
            systems::OxygenSystem.pausable(GameScene::Level),
            "oxygen_system",&["ui_transform"],)

        .with(systems::HoverInteractionSystem.pausable(GameScene::Level),
              "mouse_input_system",
              &["mouse_ray_system"],);
    let mut game = Application::new(assets_dir, MainState, game_data)?;
    game.run();
    Ok(())
}

#[derive(PartialEq)]
enum GameScene {
    Main,
    Level,
    None,
}

impl Default for GameScene {
    fn default() -> Self {
        GameScene::None
    }
}
