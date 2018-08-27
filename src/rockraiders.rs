use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Matrix4, Point3, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle, VirtualKeyCode,
    WithSpriteRender,
};

const SPRITESHEET_SIZE: (f32, f32) = (100.0, 100.0);

pub struct RockRaiders;

impl<'a, 'b> State<GameData<'a, 'b>> for RockRaiders {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let spritesheet = {
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

        //world.register::<Paddle>();

        initialise_ground(world, spritesheet);
        initialise_camera(world);
    }

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
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    //                               I assume:  >     ^  towards you
    let initial_camera_position = Point3::new(0.0, 1.0, 0.0);
    let initial_point = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, Deg(90.0))))
        .with(GlobalTransform(Matrix4::look_at(
            initial_camera_position, //eye
            initial_point,           //center
            up,                      //up-vector
        ))).build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_ground(world: &mut World, spritesheet: TextureHandle) {
    let mut ground_transform = Transform::default();

    ground_transform.translation = Vector3::new(0.0, 0.0, 0.0);

    // Build the sprite for the paddles.
    let sprite = Sprite {
        left: 0.0,
        right: 100.0,
        top: 0.0,
        bottom: 100.0,
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on ground")
        .with(GlobalTransform::default())
        .with(ground_transform)
        .build();
}
