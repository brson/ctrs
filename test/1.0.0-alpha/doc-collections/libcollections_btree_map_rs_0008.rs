fn main() {
    use std::collections::BTreeMap;
    
    let mut map = BTreeMap::new();
    map.insert(1u, "a");
    map.insert(2u, "b");
    map.insert(3u, "c");
    
    for (key, value) in map.into_iter() {
        println!("{}: {}", key, value);
    }
}
