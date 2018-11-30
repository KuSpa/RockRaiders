use amethyst::core::cgmath::{Angle, InnerSpace, Point2, Quaternion, Rad, Rotation3, Vector3};
use amethyst::core::specs::prelude::{Component, Join, Read, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::storage::DenseVecStorage;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, MovementIntent>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, mut movement_intents, mut transforms): Self::SystemData) {
        for (mut path, mut transform) in (&mut movement_intents, &mut transforms).join() {
            if path.is_empty() {
                continue;
            }

            let position = transform.translation;

            let next_destination = path[0];
            let next_destination = Vector3::from((next_destination.x, 0., next_destination.y));

            // shouldn't this be reversed?
            let direction = position - next_destination;

            (*transform).rotation = Quaternion::from_angle_y(Rad::atan2(direction.x, direction.z));

            transform.move_forward(time.delta_seconds());

            if (next_destination - transform.translation).magnitude() < time.delta_seconds() {
                path.remove(0);
            }

            // TODO remove entity if path is empty !!! lul xD
        }
    }
}

pub struct MovementIntent {
    pub path: Vec<Point2<f32>>,
}

impl Component for MovementIntent {
    type Storage = DenseVecStorage<MovementIntent>;
}

use std::ops::{Deref, DerefMut};
impl Deref for MovementIntent {
    type Target = Vec<Point2<f32>>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for MovementIntent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}
