fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
}
