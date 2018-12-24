use amethyst::ecs::{Component, DenseVecStorage, Entity, World, WriteStorage};

pub type ClickHandlerComponent = Box<dyn Clickable>;

/// This trait is meant to be used as TraitObject to enable encapsulated implementation for every possible clickable Entity.
///
/// Note, that this does nothing, if the entity has no `HoverHandler` Component.
/// Note, that the for example `Base>>TraitObject` is an other Instance of `Base` than the Base Component of an entity itself. So if there is any state, that Clickable needs to access, the preferred way would be to add a dedicated Component to the entity, which implements this Trait.
///
/// Note, that that `HoverHandler` is implemented as Component, which makes it harder to make Hoverable a supertrait of Clickable
/// This can be adapted as soon as the is the need to do so @karyon <3<3<3
pub trait Clickable: Sync + Send {
    /// This method is called, whenever the mouse hovers the entity of this component and clicks. It only is triggered on the nearest entity, that has a `Hoverable` Component as well.
    fn on_click(&self, Entity, &World);

    /// Creates a new ClickHandler, which can be stored
    fn new_click_handler(&self) -> ClickHandlerComponent;

    /// Add a ClickHandler to the entity
    fn attach_click_handler(
        &self,
        entity: Entity,
        storage: &mut WriteStorage<ClickHandlerComponent>,
    ) {
        storage.insert(entity, self.new_click_handler()).unwrap();
    }
}

impl Component for ClickHandlerComponent {
    type Storage = DenseVecStorage<ClickHandlerComponent>;
}
