fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    
    for (key, value) in map.iter_mut() {
        *value = "x";
    }
    
    for (key, value) in map.iter() {
        assert_eq!(value, &"x");
    }
}
