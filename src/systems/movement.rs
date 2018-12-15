use amethyst::{
    core::{
        nalgebra::{Point2, Real, UnitQuaternion, Vector3},
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Entity, Join, Read, System, WriteStorage,
    },
};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        WriteStorage<'a, Path>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, time, mut path_storage, mut transforms): Self::SystemData) {
        let mut removable_paths: Vec<Entity> = vec![];

        for (entity, mut path, mut transform) in
            (&entities, &mut path_storage, &mut transforms).join()
        {
            let next_destination = path[0];
            let next_destination = Vector3::<f32>::new(next_destination.x, 0.0, next_destination.y);

            // shouldn't this be reversed?
            // BUT IT WORKS, SO IT STAYS until there is a solution :)
            let direction = transform.translation() - next_destination;

            *transform.rotation_mut() = UnitQuaternion::<f32>::from_axis_angle(
                &Vector3::<f32>::y_axis(),
                Real::atan2(direction.x, direction.z),
            );

            transform.move_forward(time.delta_seconds());

            if (next_destination - transform.translation()).magnitude() < time.delta_seconds() {
                path.remove(0);
            }

            if path.is_empty() {
                removable_paths.push(entity);
            }
        }

        // we cannot do
        // removable_paths.iter().map(|&e| movement_intents.remove(*e));
        // because `map` is lazy and would do nothing...
        for e in removable_paths.iter() {
            path_storage.remove(*e);
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub path: Vec<Point2<f32>>,
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}

use std::ops::{Deref, DerefMut};
impl Deref for Path {
    type Target = Vec<Point2<f32>>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}
