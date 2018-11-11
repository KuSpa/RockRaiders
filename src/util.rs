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
pub fn find_in_vec<'a, K: Eq, R: Clone>(key: &K, dict: &'a Vec<(K, R)>) -> Option<&'a R> {
    for (dict_key, value) in dict {
        if dict_key == key {
            return Some(&value);
        }
    }
    None
}
