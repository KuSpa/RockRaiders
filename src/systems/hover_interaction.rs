use amethyst::core::GlobalTransform;
use amethyst::ecs::prelude::{
    Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, System, Write,
    WriteStorage,
};
use amethyst::renderer::{Material, TextureHandle};
use systems::MouseRay;
use ncollide3d::shape::Shape;
use amethyst::core::nalgebra::geometry::Isometry3;
use amethyst::core::nalgebra::try_convert_ref;

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
            if let Some(collision_distance) = hover_handler
                // TODO inverse isometriy of boundingboxes global transform
                .bounding_box.as_ray_cast().unwrap().toi_with_ray(&try_convert_ref(&transform.0).unwrap(), &mouse_ray.ray, true)
            {
                // Option::map_or
                // If there is no nearest collision (the `true` part), or if the current distance is shorter than the stored one, then override nearest
                if nearest.map_or(true, |(nearest_distance, _)| {
                    collision_distance < nearest_distance
                }) {
                    nearest = Some((collision_distance, entity))
                }
            }
        }
        let old_hovered_entity = (*hovered).take();
        *hovered = nearest.map(|(_, entity)| Hovered { entity });
        old_hovered_entity.map(|hovered| {
            hover_handlers
                .get_mut(hovered.entity)
                .unwrap()
                .change_materials(&hovered.entity, &mut materials)
        });
        // we cannot use `map()` here, because map would move `hovered` while only only borrowed it from the world
        if let Some(Hovered { entity: e, .. }) = *hovered {
            hover_handlers
                .get_mut(e)
                .unwrap()
                .change_materials(&e, &mut materials)
        }
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
