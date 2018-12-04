use amethyst::core::cgmath::{Point2, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, Entity, NullStorage, WriteStorage};
use amethyst::ecs::world::Entities;

pub use assetmanagement::util::*;

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
        storages: &mut RockRaiderStorages,
    ) -> Entity {
        let (
            (ref mut people_storage, ref mut transform_storage, ref mut global_transform_storage),
            ref mut asset_storages,
        ) = storages;

        let mut transform = Transform::default();
        transform.set_position(Vector3 {
            x: position.x,
            y: 0.0,
            z: position.y,
        });

        let entity = entities
            .build_entity()
            .with(RockRaider, people_storage)
            .with(transform, transform_storage)
            .with(GlobalTransform::default(), global_transform_storage)
            .build();

        insert_into_asset_storages(entity, RockRaider::asset_name(), asset_storages);
        entity
    }

    fn asset_name() -> &'static str {
        "/rock_raider/default"
    }
}

impl Component for RockRaider {
    type Storage = NullStorage<RockRaider>;
}
