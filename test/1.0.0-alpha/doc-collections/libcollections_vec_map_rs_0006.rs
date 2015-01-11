fn main() {
    use std::collections::VecMap;
    
    let mut map = VecMap::new();
    map.insert(1, "a");
    map.insert(3, "c");
    map.insert(2, "b");
    
    // Print `1: a` then `2: b` then `3: c`
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}
