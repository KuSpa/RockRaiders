use amethyst::core::cgmath::{Point2, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, Entity, NullStorage, WriteStorage};
use amethyst::ecs::world::Entities;
use amethyst::prelude::*;

pub use assetmanagement::util::*;

pub struct RockRaider;

type Storages<'a> = (
    WriteStorage<'a, RockRaider>,
    WriteStorage<'a, Transform>,
    WriteStorage<'a, GlobalTransform>,
    RequiredAssetStorages<'a>,
);

impl RockRaider {
    //note: this is a type alias for Read<'a, EntityRes>
    pub fn instantiate(
        entities: &mut Entities,
        position: Point2<f32>,
        storages: &mut Storages,
    ) -> Entity {
        let (
            ref mut people_storage,
            ref mut transform_storage,
            ref mut global_transform_storage,
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

        insert_into_storages(entity, RockRaider::asset_name(), asset_storages);
        entity
    }

    fn asset_name() -> &'static str {
        "/rock_raider/default"
    }
}

impl Component for RockRaider {
    type Storage = NullStorage<RockRaider>;
}

impl Default for RockRaider {
    fn default() -> Self {
        RockRaider
    }
}
