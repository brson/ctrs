fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    match map.get_mut(&1) {
        Some(x) => *x = "b",
        None => (),
    }
    assert_eq!(map[1], "b");
}
