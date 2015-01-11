fn main() {
    use std::collections::VecMap;
    let map: VecMap<String> = VecMap::with_capacity(10);
    assert!(map.capacity() >= 10);
}
