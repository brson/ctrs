fn main() {
    use std::collections::HashMap;
    
    let mut map: HashMap<int, int> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
}
