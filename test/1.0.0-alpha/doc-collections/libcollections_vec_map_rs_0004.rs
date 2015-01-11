fn main() {
    use std::collections::VecMap;
    let mut map: VecMap<&str> = VecMap::new();
    map.reserve_len(10);
    assert!(map.capacity() >= 10);
}
