use amethyst::core::cgmath::{MetricSpace, Point3};
use amethyst::core::GlobalTransform;
use amethyst::ecs::prelude::{
    Component, DenseVecStorage, Entities, Entity, Join, Read, ReadExpect, ReadStorage, System,
    Write, WriteStorage,
};
use amethyst::renderer::Material;
use collision::primitive::Primitive3;
use collision::{ContinuousTransformed, Ray3};
use systems::MouseRay;

pub struct HoverInteractionSystem;

impl<'a> System<'a> for HoverInteractionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseRay>,
        ReadStorage<'a, GlobalTransform>,
        WriteStorage<'a, HoverHandler>,
        WriteStorage<'a, Material>,
        Write<'a, Option<Hovered>>,
    );

    fn run(
        &mut self,
        (entities, mouse_ray, transforms, mut hover_handlers, mut materials, mut hovered): Self::SystemData,
    ) {
        let mut nearest = None;
        for (entity, hover_handler, transform) in
            (&*entities, &mut hover_handlers, &transforms).join()
        {
            if let Some(point) = hover_handler
                .bounding_box
                .intersection_transformed(&mouse_ray.ray, &transform.0)
            {
                let collision_distance = mouse_ray.ray.origin.distance2(point);
                if nearest.map_or(true, |(nearest_distance, _, _)| {
                    collision_distance < nearest_distance
                }) {
                    nearest = Some((collision_distance, point, entity))
                }
            }
        }

        let old_hovered_entity = (*hovered).clone();

        *hovered = nearest.map(|(_, point, entity)| Hovered {
            entity: entity,
            point: point,
        });

        match (&old_hovered_entity, &*hovered) {
            (Some(Hovered { entity: a, .. }), Some(Hovered { entity: b, .. })) if a != b => {
                // a new entity was hovered. Revert the old's Material and update the new one's
                hover_handlers
                    .get_mut(*a)
                    .unwrap()
                    .revert_hover_effect(*a, &materials);
                hover_handlers
                    .get_mut(*b)
                    .unwrap()
                    .do_hover_effect(*b, &materials);
            }
            (Some(Hovered { entity: a, .. }), None) => {
                // hover stopped, no new hover
                hover_handlers
                    .get_mut(*a)
                    .unwrap()
                    .revert_hover_effect(*a, &materials);
            }
            (None, Some(Hovered { entity: b, .. })) => {
                //hover started
                hover_handlers
                    .get_mut(*b)
                    .unwrap()
                    .do_hover_effect(*b, &materials);
            }
            _ => (), //either the same entity or None at all -> no need to update Mat
        }
    }
}

// Only entities with this Component can be hovered. Other Entities will be ignored
pub struct HoverHandler {
    pub bounding_box: Primitive3<f32>,
    pub hover: Material,
    revert: Option<Material>,
}

impl HoverHandler {
    fn revert_hover_effect(&mut self, entity: Entity, materials: &WriteStorage<Material>) {
        //TODO
    }
    fn do_hover_effect(&mut self, entity: Entity, materials: &WriteStorage<Material>) {
        //TODO
    }
}

impl Component for HoverHandler {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Hovered {
    pub entity: Entity,
    pub point: Point3<f32>,
}
