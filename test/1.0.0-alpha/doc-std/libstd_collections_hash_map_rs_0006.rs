fn main() {
    use std::collections::HashMap;
    let map: HashMap<int, int> = HashMap::with_capacity(100);
    assert!(map.capacity() >= 100);
}
