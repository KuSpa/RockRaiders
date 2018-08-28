use amethyst::core::specs::prelude::{Read, ReadStorage, System};
use amethyst::input::InputHandler;
use amethyst::renderer::Camera;

pub struct CameraMovementSystem;

impl<'a> System<'a> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (cam, input): Self::SystemData) {
        //TODO
    }
}
