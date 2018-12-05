/// rotates the array clockwise
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
pub fn amount_in<T: Join + GenericReadStorage>(storage: T) -> usize {
    storage.join().count()
}
