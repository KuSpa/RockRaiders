use amethyst::ecs::World;
use amethyst::ecs::{Component, DenseVecStorage, Entity};

/// This trait is meant to be used as TraitObject to enable encapsulated implementation for every possible clickable Entity.
///
/// Note, that this does nothing, if the entity has no `HoverHandler` Component.
/// Note, that the for example `Base>>TraitObject` is an other Instance of `Base` than the Base Component of an entity itself. So if there is any state, that Clickable needs to access, the preferred way would be to add a dedicated Component to the entity, which implements this Trait.
///
/// Note, that that `HoverHandler` is implemented as Component, which makes it harder to make Hoverable a supertrait of Clickable
/// This can be adapted as soon as the is the need to do so @karyon <3<3<3
pub trait Clickable: Sync + Send {
    fn on_click(&self, Entity, &World);
}

impl Component for Box<dyn Clickable> {
    type Storage = DenseVecStorage<Box<dyn Clickable>>;
}
