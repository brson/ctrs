fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(1u, "a");
    match map.get_mut(&1) {
        Some(x) => *x = "b",
        None => (),
    }
    assert_eq!(map[1], "b");
}
