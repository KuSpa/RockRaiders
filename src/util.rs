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
pub fn find_in_vec<K: Eq, R: Clone>(key: &K, dict: &Vec<(K, R)>) -> Option<R> {
    for pair in dict {
        if pair.0 == *key {
            return Some(pair.1.clone());
        }
    }
    None
}
