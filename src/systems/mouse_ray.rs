use amethyst::core::nalgebra::{Matrix4, Point3, Vector3, Vector4};
use amethyst::core::specs::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::renderer::ActiveCamera;
use amethyst::renderer::{Camera, ScreenDimensions};
use ncollide3d::query::Ray;

pub struct MouseRaySystem;

impl<'s> System<'s> for MouseRaySystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        Option<ReadExpect<'s, ActiveCamera>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Write<'s, MouseRay>,
    );
    fn run(&mut self, (input, cam, dims, camera, transform, mut mouse_ray): Self::SystemData) {
        if cam.is_none() {
            return;
        }

        let active_cam = cam.unwrap();
        match (
            input.mouse_position(),
            camera.get(active_cam.entity),
            transform.get(active_cam.entity),
        ) {
            (Some((x, y)), Some(camera), Some(transform)) => {
                let dpi_factor = dims.hidpi_factor() as f32;
                // May break, when camera should ever be child of an other transform.
                mouse_ray.ray.origin = Point3::from(transform.translation().clone());
                mouse_ray.ray.dir = from_window_space(
                    (x as f32, y as f32),
                    (dims.width() * dpi_factor, dims.height() * dpi_factor),
                    camera.proj,
                    transform.matrix(),
                );
            }
            _ => (),
        }
    }
}

fn from_window_space(
    (position_x, position_y): (f32, f32),
    (width, height): (f32, f32),
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
) -> Vector3<f32> {
    let mut v = Vector4::new(
        2. * position_x / width - 1.,
        2. * (height - position_y) / height - 1.,
        0.,
        0.,
    );
    v = proj
        .try_inverse()
        .expect("Could not build inverse matrix of ")
        * v;
    v.z = -1.;
    v = view * v;
    v = v.normalize();
    Vector3::new(v.x, v.y, v.z)
}

pub struct MouseRay {
    pub ray: Ray<f32>,
}

impl Default for MouseRay {
    fn default() -> Self {
        MouseRay {
            ray: Ray::new(Point3::new(0., 0., 0.), Vector3::new(1., 1., 1.)),
        }
    }
}
