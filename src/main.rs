extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::GlobalTransform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, DisplayConfig, DrawFlat, Event, MaterialTextureSet, Pipeline, PngFormat, PosNormTex,
    Projection, RenderBundle, Sprite, SpriteSheet, SpriteSheetHandle, Stage, Texture,
    VirtualKeyCode,
};

struct RockRaiders;

impl<'a, 'b> State<GameData<'a, 'b>> for RockRaiders {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        //BAGUETTE FIND WHY THIS CAN BE MUT REFErENCE
        println!(">>> initializing Main struct");
        let handle = load_sprite(world);
        init_ground(world, handle);
    }
}

fn load_sprite(world: &mut World) -> SpriteSheetHandle {
    println!("starting texture");
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "ground.png",
            PngFormat,
            Default::default(),
            (),
            &texture_storage,
        )
    };
    println!("got handle");
    let texture_id = 0;
    //--------------------------
    let mut material_texture_set = MaterialTextureSet::new();
    material_texture_set.insert(texture_id, texture_handle);
    world.add_resource(material_texture_set);

    let asset_store = AssetStorage::<SpriteSheet>::new();
    world.add_resource(asset_store);

    //let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    //println!("got worldhandle");
    //material_texture_set.insert(texture_id, texture_handle);
    //--------------------------
    println!("got texset");
    let sprite = Sprite {
        left: 0.0,
        right: 1.0,
        top: 0.0,
        bottom: 1.0,
    };
    println!("got sprite");

    let sprite_sheet = SpriteSheet {
        texture_id,
        sprites: vec![sprite],
    };
    println!("got spritesheet");

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();

        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        println!("loaderstorage");
        loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
    };
    println!("got spritehandle");

    sprite_sheet_handle
}

fn init_ground(world: &World, sprite_sheet_handle: SpriteSheetHandle) {}

fn initialise_camera(world: &mut World) {
    use amethyst::core::cgmath::{Matrix4, Vector3};
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 100.0, 100.0, 0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&config_path);

    let assets_dir = format!("{}/textures/", env!("CARGO_MANIFEST_DIR"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.2, 0.2, 0.2, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::build(assets_dir, RockRaiders)?.build(game_data)?;
    game.run();
    Ok(())
}

struct Ground;
impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}
