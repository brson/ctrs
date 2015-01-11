fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    map.insert(3, "c");
    map.insert(2, "b");
    
    // Not possible with .iter()
    let vec: Vec<(uint, &str)> = map.into_iter().collect();
    
    assert_eq!(vec, vec![(1, "a"), (2, "b"), (3, "c")]);
}
