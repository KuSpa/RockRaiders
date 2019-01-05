extern crate amethyst;
#[macro_use]
extern crate amethyst_derive;
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
mod main_state;
mod systems;
mod ui;
mod util;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
};

use eventhandling::{GameEvent, GameEventReader};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    use main_state::MainState;

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
        .with(
            eventhandling::MouseRaySystem.pausable(GameScene::Level),
            "mouse_ray_system",
            &[],
        )
        .with(
            ui::UiMapUpdateSystem.pausable(GameScene::Level),
            "ui_map_update_system",
            &["ui_transform"],
        )
        .with(
            ui::UiRockRaiderSystem.pausable(GameScene::Level),
            "ui_rr_update_system",
            &["ui_transform", "ui_map_update_system"],
        )
        .with(
            systems::MovementSystem.pausable(GameScene::Level),
            "movement_system",
            &["transform_system"],
        )
        .with(
            eventhandling::CameraMovementSystem.pausable(GameScene::Level),
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
            "oxygen_system",
            &["ui_transform"],
        )
        .with(
            eventhandling::HoverInteractionSystem.pausable(GameScene::Level),
            "mouse_input_system",
            &["mouse_ray_system"],
        );

    let mut game =
        CoreApplication::<_, GameEvent, GameEventReader>::new(assets_dir, MainState, game_data)?;
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
