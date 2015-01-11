fn main() {
    use std::collections::VecMap;
    
    let mut a = VecMap::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());
}
