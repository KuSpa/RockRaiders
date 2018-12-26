use amethyst::{
    core::specs::{storage::GenericReadStorage, Entities, Entity, Join, World},
    shred::Resource,
    ui::UiTransform,
};

/// Rotates the array clockwise.
pub fn rotate_3x3<T: Clone>(input: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    let mut result = input.clone();

    for x in 0..3 {
        for y in 0..3 {
            result[x][y] = input[3 - y - 1][x].clone()
        }
    }
    result
}

/// Returns the amount of `Components` in a `Storage`.
pub fn amount_in<T: Join + GenericReadStorage>(storage: T) -> usize {
    storage.join().count()
}

/// Copy of https://www.amethyst.rs/doc/latest/doc/src/amethyst_ui/transform.rs.html#11-14
/// Since UiFinder has its own handle to the UiTransform storage, it is impossible to use both in a system, because the storage would be borrowed twice. The method is extracted here
pub fn find_ui_by_name<'a, T: Join<Type = &'a UiTransform>>(
    id: &str,
    entities: &Entities,
    ui_transforms: T,
) -> Option<Entity> {
    (entities, ui_transforms)
        .join()
        .find(|(_, transform)| (transform).id == id)
        .map(|(entity, _)| entity)
}

/// Updates a resource if the resource is Default or doesn't exist.
// TODO remove with merge of https://github.com/slide-rs/specs/pull/522
pub fn add_resource_soft<T: Resource + Default + PartialEq>(world: &mut World, res: T) {
    if !world.res.has_value::<T>() {
        if (*world.read_resource::<T>()) == T::default() {
            *world.write_resource() = res
        }
    } else {
        world.add_resource::<T>(res)
    }
}
