use amethyst::ecs::World;
use amethyst::ecs::{Component, DenseVecStorage, Entity};
use entities::buildings::Base;

///
/// This trait is meant to be used as TraitObject to enable encapsulated implementation for every possible clickable Entity
/// Note, that this does nothing since `HoverHandler` is implemented as Component, which makes it harder to make Hoverable a supertrait of Clickable
/// This can be adapted as soon as the is the need to do so
pub trait Clickable: Sync + Send {
    fn on_click(&self, &Entity, &World);
}

impl Component for Box<dyn Clickable> {
    type Storage = DenseVecStorage<Box<dyn Clickable>>;
}

impl Clickable for Base {
    fn on_click(&self, _entity: &Entity, world: &World) {
        let mut storages = world.system_data();
        let entities = world.entities();
        self.spawn_rock_raider(&entities, &mut storages);
    }
}
