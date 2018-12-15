use amethyst::{
    core::{nalgebra::try_convert_ref, GlobalTransform},
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, System, Write,
        WriteStorage,
    },
    renderer::{Material, TextureHandle},
};
use ncollide3d::shape::Shape;
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
        let mut nearest_dist = None;
        let mut nearest_entity = None;
        for (entity, hover_handler, transform) in
            (&*entities, &mut hover_handlers, &transforms).join()
        {
            if let Some(collision_distance) = hover_handler
                .bounding_box
                .as_ray_cast()
                .unwrap()
                .toi_with_ray(
                    &try_convert_ref(&transform.0).unwrap(),
                    &mouse_ray.ray,
                    true,
                )
            {
                if let Some(ref mut dist) = nearest_dist {
                    if collision_distance < *dist {
                        *dist = collision_distance;
                        nearest_entity = Some(entity);
                    }
                } else {
                    nearest_dist = Some(collision_distance);
                    nearest_entity = Some(entity);
                }
            }
        }

        let old_hovered_entity = (*hovered).take();
        old_hovered_entity.map(|hovered| {
            hover_handlers
                .get_mut(hovered.entity)
                .unwrap()
                .change_materials(&hovered.entity, &mut materials)
        });
        // we cannot use `map()` here, because map would move `hovered` while only only borrowed it from the world

        *hovered = nearest_entity.map(|entity| {
            hover_handlers
                .get_mut(entity)
                .unwrap()
                .change_materials(&entity, &mut materials);
            Hovered { entity }
        });
    }
}

// Only entities with this Component can be hovered. Other Entities will be ignored
pub struct HoverHandler {
    pub bounding_box: Box<dyn Shape<f32>>,
    // when hovered, the original `TextureHandle` will be stored here.
    pub hover: TextureHandle,
}

impl HoverHandler {
    fn change_materials(&mut self, entity: &Entity, materials: &mut WriteStorage<Material>) {
        let mat = materials.get_mut(*entity).unwrap();
        let texture_handle = mat.albedo.clone();
        mat.albedo = self.hover.clone();
        self.hover = texture_handle;
    }
}

impl Component for HoverHandler {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Hovered {
    pub entity: Entity,
}
