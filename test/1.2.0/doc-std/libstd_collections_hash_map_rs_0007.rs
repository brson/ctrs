fn main() {
    use std::collections::HashMap;
    let map: HashMap<isize, isize> = HashMap::with_capacity(100);
    assert!(map.capacity() >= 100);
}
