use amethyst::{
    core::{
        nalgebra::{Unit, Vector3, Vector4},
        specs::prelude::{Read, ReadExpect, System, WriteStorage},
        timing::Time,
        transform::Transform,
    },
    input::InputHandler,
    renderer::{ActiveCamera, ScreenDimensions},
};

pub struct CameraMovementSystem;

impl<'a> System<'a> for CameraMovementSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<String, String>>,
        Option<ReadExpect<'a, ActiveCamera>>,
        ReadExpect<'a, ScreenDimensions>,
    );

    fn run(&mut self, (time, mut transforms, input, active_cam, screen_size): Self::SystemData) {
        if let (Some((mouse_x, mouse_y)), Some(cam)) = (input.mouse_position(), active_cam) {
            let (mut x, mut z) = (0.0, 0.0);
            let mouse_x = mouse_x;
            let mouse_y = mouse_y;

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

            let mut dir = Vector4::new(x, 0.0, z, 0.0);
            if !(dir.magnitude() == 0.0) {
                let mut transform = transforms.get_mut(cam.entity).unwrap();
                dir = transform.matrix() * dir;
                let move_dir = Unit::<Vector3<f32>>::new_normalize(Vector3::new(dir.x, 0.0, dir.z));
                transform.move_along_global(move_dir, time.delta_seconds());
            }
        }
    }
}
