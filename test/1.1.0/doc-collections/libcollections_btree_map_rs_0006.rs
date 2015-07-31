fn main() {
    use std::collections::BTreeMap;
    
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    
    for (key, value) in map.into_iter() {
        println!("{}: {}", key, value);
    }
}
