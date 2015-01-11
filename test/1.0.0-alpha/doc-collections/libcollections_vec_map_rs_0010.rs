fn main() {
    use std::collections::VecMap;
    
    let mut a = VecMap::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
}
