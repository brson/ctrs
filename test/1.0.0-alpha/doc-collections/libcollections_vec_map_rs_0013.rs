fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
}
