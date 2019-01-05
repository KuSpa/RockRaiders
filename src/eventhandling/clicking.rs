use amethyst::ecs::{Component, DenseVecStorage, Entity, World};

pub type ClickHandlerComponent = Box<dyn Clickable>;

/// This trait is meant to be used as TraitObject to enable encapsulated implementation for every possible clickable Entity.
///
/// Note, that this does nothing, if the entity has no `HoverHandler` Component.
/// Note, that the for example `Base>>TraitObject` is an other Instance of `Base` than the Base Component of an entity itself. So if there is any state, that Clickable needs to access, the preferred way would be to add a dedicated Component to the entity, which implements this Trait.
pub trait Clickable: Sync + Send {
    /// This method is called, whenever the mouse hovers the entity of this component and clicks. It only is triggered on the nearest entity, that has a `Hoverable` Component as well.
    fn on_click(&self, Entity, &World);
}

impl Component for ClickHandlerComponent {
    type Storage = DenseVecStorage<ClickHandlerComponent>;
}
