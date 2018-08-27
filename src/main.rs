extern crate amethyst;

mod rockraiders;

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawFlat, PosTex};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use rockraiders::RockRaiders;

    let path = format!("{}/resources/display.ron", env!("CARGO_MANIFEST_DIR"));

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = format!("{}/", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?;
    let mut game = Application::new(assets_dir, RockRaiders, game_data)?;
    game.run();
    Ok(())
}
