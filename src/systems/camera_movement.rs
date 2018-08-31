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
        //TODO use window settings!!!
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            //X value of the mouse
            if mouse_x < 20.0 {
                x -= 1.0;
            } else if mouse_x > 480.0 {
                x += 1.0;
            }

            //Y value of the mouse === Z movement of the camera
            if mouse_y < 20.0 {
                z -= 1.0;
            } else if mouse_y > 480.0 {
                z += 1.0;
            }
        }

        // Keep this for debugging purposes
        //for pressed_key in input.keys_that_are_down() {
        //    match pressed_key {
        //        VirtualKeyCode::W => z -= 1.0,
        //        VirtualKeyCode::S => z += 1.0,
        //        VirtualKeyCode::A => x -= 1.0,
        //        VirtualKeyCode::D => x += 1.0,
        //        _ => (),
        //    }
        //}
        let dir = Vector3::new(x, 0.0, z);
        info!("{:?}", dir);
        if !dir.is_zero() {
            for (transform, _) in (&mut transforms, &cam).join() {
                transform.move_along_local(dir, time.delta_seconds() * ((x.abs() + z.abs()) / 2.0));
            }
        }
    }
}
