fn main() {
    use std::collections::BTreeMap;
    
    let mut map = BTreeMap::new();
    map.insert(1u, "a");
    map.insert(2u, "b");
    map.insert(3u, "c");
    
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
    
    let (first_key, first_value) = map.iter().next().unwrap();
    assert_eq!((*first_key, *first_value), (1u, "a"));
}
