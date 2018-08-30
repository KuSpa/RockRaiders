use amethyst::core::specs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;

use amethyst::core::cgmath::prelude::Zero;
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
        let (mut x, mut z) = (0.0, 0.0);
        //TODO - use mouse for movement
        for pressed_key in input.keys_that_are_down() {
            match pressed_key {
                VirtualKeyCode::W => z -= 1.0,
                VirtualKeyCode::S => z += 1.0,
                VirtualKeyCode::A => x -= 1.0,
                VirtualKeyCode::D => x += 1.0,
                _ => (),
            }
        }
        let dir = Vector3::new(x, 0.0, z);
        if !dir.is_zero() {
            for (transform, _) in (&mut transforms, &cam).join() {
                transform.move_along_local(dir, time.delta_seconds() * 20.0);
            }
        }
    }
}
