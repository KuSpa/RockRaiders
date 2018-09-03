use amethyst::core::specs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;

use amethyst::core::cgmath::prelude::Zero;
use amethyst::core::cgmath::{Vector3, Vector4};
use amethyst::renderer::{Camera, ScreenDimensions};

pub struct CameraMovementSystem;

impl<'a> System<'a> for CameraMovementSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<String, String>>,
        ReadStorage<'a, Camera>,
        ReadExpect<'a, ScreenDimensions>,
    );

    fn run(&mut self, (time, mut transforms, input, cam, screen_size): Self::SystemData) {
        let (mut x, mut z) = (0.0, 0.0);
        //TODO = 20 as magic number in .ron spec file?
        //       Offset to borders as speed? for a smoother feeling?

        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            //X value of the mouse
            if mouse_x < 20.0 {
                x -= 1.0;
            } else if mouse_x > (screen_size.width() - 20.0).into() {
                x += 1.0;
            }

            //Y value of the mouse === Z movement of the camera
            if mouse_y < 20.0 {
                z -= 1.0;
            } else if mouse_y > (screen_size.height() - 20.0).into() {
                z += 1.0;
            }
        }

        let mut dir = Vector4::new(x, 0.0, z, 0.0);
        if !dir.is_zero() {
            for (transform, _) in (&mut transforms, &cam).join() {
                dir = transform.matrix() * dir;
                let move_dir = Vector3::new(dir.x, 0.0, dir.z);

                transform.move_along_global(
                    move_dir,
                    time.delta_seconds() * ((x.abs() + z.abs()) / 2.0),
                );
            }
        }
    }
}
