fn main() {
    use std::collections::BTreeMap;
    
    let mut map = BTreeMap::new();
    map.insert(1u, "a");
    match map.get_mut(&1) {
        Some(x) => *x = "b",
        None => (),
    }
    assert_eq!(map[1], "b");
}
