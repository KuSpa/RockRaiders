use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, Entities, Entity, NullStorage, World, WriteStorage},
    renderer::{PngFormat, Texture, TextureMetadata},
};
use assetmanagement::util::*;
use assetmanagement::{util::attach_assets, MeshManager, TextureManager};
use eventhandling::{ClickHandlerComponent, Clickable, HoverHandlerComponent, SimpleHoverHandler};
use level::SelectedRockRaider;
use ncollide3d::shape::Cuboid;

/// A Tag to indicate the entity as `RockRader`
/// `RockRaider`a are the little moving people, that the player can control to do certain tasks ;).
#[derive(Default)]
pub struct RockRaider;

impl RockRaider {
    pub fn instantiate(
        entities: &Entities, //note: this is a type alias for Read<'a, EntityRes>
        position: Point2<f32>,
        (ref mut rock_raider_storage,ref  mut transform_storage,ref  mut global_transform_storage): &mut (
            WriteStorage<RockRaider>,
            WriteStorage<Transform>,
            WriteStorage<GlobalTransform>,
        ),
        texture_storages: &mut TextureStorages,
        mesh_storages: &mut MeshStorages,
        loader: &Loader,
        mut hover_storage: WriteStorage<HoverHandlerComponent>,
        mut click_storage: WriteStorage<ClickHandlerComponent>,
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_position(Vector3::new(position.x, 0.0, position.y));

        let entity = entities
            .build_entity()
            .with(RockRaider, rock_raider_storage)
            .with(transform, transform_storage)
            .with(GlobalTransform::default(), global_transform_storage)
            .build();

        attach_assets(
            entity,
            RockRaider::asset_name(),
            &loader,
            texture_storages,
            mesh_storages,
        );

        let (ref mut tex_manager, ref mut tex_storage, ref _mat_storage) = texture_storages;

        let handler = Self::new_hover_handler(tex_manager, &loader, tex_storage);
        hover_storage.insert(entity, handler).unwrap();

        RockRaider.attach_click_handler(entity, &mut click_storage);

        entity
    }

    pub fn new_hover_handler(
        tex_manager: &mut TextureManager,
        loader: &Loader,
        mut tex_storage: &mut AssetStorage<Texture>,
    ) -> HoverHandlerComponent {
        let hover_mat =
            tex_manager.get_handle_or_load("/rock_raider/default_hover", &loader, &mut tex_storage);

        let bounding_box = Cuboid::new(Vector3::new(0.21, 0.18, 0.1));
        Box::new(SimpleHoverHandler::new(bounding_box, hover_mat))
    }

    fn asset_name() -> &'static str {
        "/rock_raider/default"
    }
}

impl Component for RockRaider {
    type Storage = NullStorage<RockRaider>;
}

impl Clickable for RockRaider {
    fn on_click(&self, entity: Entity, world: &World) {
        *world.write_resource::<Option<SelectedRockRaider>>() = Some(SelectedRockRaider(entity));
    }

    fn new_click_handler(&self) -> ClickHandlerComponent {
        Box::new(RockRaider) as ClickHandlerComponent
    }
}
