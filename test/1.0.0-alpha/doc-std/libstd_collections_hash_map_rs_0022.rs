fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(1u, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
}
