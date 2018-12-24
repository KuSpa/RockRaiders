use amethyst::{
    core::{
        nalgebra::{try_convert, Isometry, Isometry3, Translation3},
        GlobalTransform,
    },
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, System, World,
        Write, WriteStorage,
    },
    renderer::{Material, TextureHandle},
};

use eventhandling::MouseRay;
use ncollide3d::shape::Shape;

pub struct HoverInteractionSystem;

impl<'a> System<'a> for HoverInteractionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseRay>,
        ReadStorage<'a, GlobalTransform>,
        WriteStorage<'a, HoverHandlerComponent>,
        Write<'a, Option<Hovered>>,
    );

    fn run(
        &mut self,
        (entities, mouse_ray, transforms, mut hover_handlers, mut hovered): Self::SystemData,
    ) {
        let mut nearest_dist = None;
        let mut nearest_entity = None;
        for (entity, hover_handler, transform) in
            (&*entities, &mut hover_handlers, &transforms).join()
        {
            if let Some(collision_distance) = {
                // the mesh model has its pivot point on the bottom, the collider have their pivots in the middle.
                // to adjust the position we need to move the collision shapes a bit up
                let offset: Translation3<f32> = Translation3::new(
                    0.0,
                    hover_handler
                        .bounding_box()
                        .aabb(&Isometry::identity())
                        .half_extents()
                        .y,
                    0.0,
                );
                let mut translation: Isometry3<f32> = try_convert(transform.0).unwrap();
                translation.append_translation_mut(&offset);

                hover_handler
                    .bounding_box()
                    .as_ray_cast()
                    .unwrap()
                    .toi_with_ray(&translation, &mouse_ray.ray, true)
            } {
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
        *hovered = nearest_entity.map(|entity| Hovered { entity });
    }
}

#[derive(Clone)]
pub struct Hovered {
    pub entity: Entity,
}

pub type HoverHandlerComponent = Box<dyn Hoverable>;

/// This trait is meant to be used as TraitObject to enable encapsulated implementation for every possible hoverable Entity.
/// The TraitObjects will mark enties that are hoverable.
///
/// Note that only hoverable entities are `Clickable`,
pub trait Hoverable: Sync + Send {
    /// This method is called, whenever the mouse hovers the entity of this component. It only is triggered on the nearest entity, that has a `Hoverable` Comonent as well.
    fn on_hover_start(&mut self, _: Entity, _: &World) {}

    /// This method os called, when the hovering stops :)
    fn on_hover_stop(&mut self, _: Entity, _: &World) {}

    fn bounding_box(&self) -> &Box<dyn Shape<f32>>;
}

impl Component for HoverHandlerComponent {
    type Storage = DenseVecStorage<HoverHandlerComponent>;
}

/// A Hoverhandler that does nothing on hover. Used to enable clicking for the Entity
pub struct NoEffectHoverHandler {
    /// The bounding box, that needs to collide with the `MouseRay` on order to be considered as hovered
    bounding_box: Box<dyn Shape<f32>>,
}

impl NoEffectHoverHandler {
    /// Creates a new Handler with a given bounding box
    pub fn new<T: Shape<f32>>(bounding_box: T) -> Self {
        Self {
            bounding_box: Box::new(bounding_box) as Box<dyn Shape<f32>>,
        }
    }
}

impl Hoverable for NoEffectHoverHandler {
    fn bounding_box(&self) -> &Box<dyn Shape<f32>> {
        &self.bounding_box
    }
}

/// A Hoverhandler that switches materials on hover.
pub struct SimpleHoverHandler {
    /// The bounding box, that needs to collide with the `MouseRay` on order to be considered as hovered
    bounding_box: Box<dyn Shape<f32>>,
    /// The hover texture of the entity. When hovered, the default texturehandle is stored here instead.
    texture: TextureHandle,
}

impl SimpleHoverHandler {
    /// Creates a new Handler with a given bounding box and swap textures
    pub fn new<T: Shape<f32>>(bounding_box: T, handle: TextureHandle) -> Self {
        Self {
            bounding_box: Box::new(bounding_box) as Box<dyn Shape<f32>>,
            texture: handle,
        }
    }
}

impl Hoverable for SimpleHoverHandler {
    fn bounding_box(&self) -> &Box<dyn Shape<f32>> {
        &self.bounding_box
    }

    fn on_hover_start(&mut self, entity: Entity, world: &World) {
        let mut materials = world.write_storage::<Material>();
        let mat = materials.get_mut(entity).unwrap();
        let texture_handle = mat.albedo.clone();
        mat.albedo = self.texture.clone();
        self.texture = texture_handle;
    }

    fn on_hover_stop(&mut self, entity: Entity, world: &World) {
        let mut materials = world.write_storage::<Material>();
        let mat = materials.get_mut(entity).unwrap();
        let texture_handle = mat.albedo.clone();
        mat.albedo = self.texture.clone();
        self.texture = texture_handle;
    }
}
