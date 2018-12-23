use amethyst::{
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, Entities, Entity, NullStorage, World, WriteStorage},
};
use assetmanagement::util::*;
use eventhandling::Clickable;
use level::SelectedRockRaider;
use ncollide3d::shape::{Cuboid, Shape};
use systems::HoverHandler;

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
        hover_storage: WriteStorage<HoverHandler>,
        mut click_storage: WriteStorage<Box<dyn Clickable>>,
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
            tex_manager,
            _mat_storage,
            tex_storage,
            _default_mat,
        ) = asset_storages;

        add_hover_handler(
            entity,
            Self::asset_name(),
            Box::new(Cuboid::new(Vector3::<f32>::new(0.21, 0.18, 0.1))) as Box<dyn Shape<f32>>,
            &mut (loader, tex_manager, tex_storage, hover_storage),
        );
        let handler = Box::new(RockRaider) as Box<dyn Clickable>;
        click_storage.insert(entity, handler).unwrap();

        entity
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
}
