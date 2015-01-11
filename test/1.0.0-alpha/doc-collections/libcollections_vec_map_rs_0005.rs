fn main() {
    use std::collections::VecMap;
    let mut map: VecMap<&str> = VecMap::new();
    map.reserve_len_exact(10);
    assert!(map.capacity() >= 10);
}
