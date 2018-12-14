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

use amethyst::ecs::{storage::GenericReadStorage, Join};

/// Returns the amount of `Components` in a `Storage`.
pub fn amount_in<T: Join + GenericReadStorage>(storage: T) -> usize {
    storage.join().count()
}

use amethyst::ecs::{Entities, Entity};
use amethyst::ui::UiTransform;

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
