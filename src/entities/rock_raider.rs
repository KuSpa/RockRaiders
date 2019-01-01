use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, Entities, Entity, NullStorage, World, WriteStorage},
    renderer::{PngFormat, Texture, TextureMetadata},
};
use assetmanagement::{util::*, AssetManager};
use eventhandling::{ClickHandlerComponent, Clickable, HoverHandlerComponent, SimpleHoverHandler};
use level::SelectedRockRaider;
use ncollide3d::shape::Cuboid;

/// A Tag to indicate the entity as `RockRader`
/// `RockRaider`a are the little moving people, that the player can control to do certain tasks ;).
#[derive(Default)]
pub struct RockRaider;

pub type RockRaiderStorages<'a> = (
    (
        WriteStorage<'a, RockRaider>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, GlobalTransform>,
    ),
    AssetStorages<'a>,
);

impl RockRaider {
    pub fn instantiate(
        entities: &Entities, //note: this is a type alias for Read<'a, EntityRes>
        position: Point2<f32>,
        rr_storages: RockRaiderStorages,
        mut hover_storage: WriteStorage<HoverHandlerComponent>,
        mut click_storage: WriteStorage<ClickHandlerComponent>,
    ) -> Entity {
        let (
            (mut rock_raider_storage, mut transform_storage, mut global_transform_storage),
            mut asset_storages,
        ) = rr_storages;

        let mut transform = Transform::default();
        transform.set_position(Vector3::new(position.x, 0.0, position.y));

        let entity = entities
            .build_entity()
            .with(RockRaider, &mut rock_raider_storage)
            .with(transform, &mut transform_storage)
            .with(GlobalTransform::default(), &mut global_transform_storage)
            .build();

        insert_into_asset_storages(entity, RockRaider::asset_name(), &mut asset_storages);

        let (
            loader,
            _mesh_manager,
            _mesh_handles,
            _mesh_storage,
            mut tex_manager,
            _mat_storage,
            mut tex_storage,
            _default_mat,
        ) = asset_storages;

        let handler = Self::new_hover_handler(&loader, &mut tex_manager, &mut tex_storage);
        hover_storage.insert(entity, handler).unwrap();

        RockRaider.attach_click_handler(entity, &mut click_storage);

        entity
    }

    pub fn new_hover_handler(
        loader: &Loader,
        tex_manager: &mut AssetManager<Texture>,
        mut tex_storage: &mut AssetStorage<Texture>,
    ) -> HoverHandlerComponent {
        let hover_mat = tex_manager.get_asset_handle_or_load(
            "/rock_raider/default_hover",
            PngFormat,
            TextureMetadata::srgb(),
            &mut tex_storage,
            &loader,
        );

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
