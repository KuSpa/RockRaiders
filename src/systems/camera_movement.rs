use amethyst::core::specs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;

use amethyst::core::cgmath::Vector3;
use amethyst::renderer::{Camera, VirtualKeyCode};

pub struct CameraMovementSystem;

impl<'a> System<'a> for CameraMovementSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<String, String>>,
        ReadStorage<'a, Camera>,
    );

    fn run(&mut self, (time, mut transforms, input, cam): Self::SystemData) {
        let (transform, _) = (&mut transforms, &cam).join().nth(1).unwrap();

        //todo make pretty
        for pressed_key in input.keys_that_are_down() {
            match pressed_key {
                VirtualKeyCode::W => {}
                VirtualKeyCode::S => {}
                VirtualKeyCode::A => {}
                VirtualKeyCode::D => {}
                _ => (),
            }
        }
    }
}
