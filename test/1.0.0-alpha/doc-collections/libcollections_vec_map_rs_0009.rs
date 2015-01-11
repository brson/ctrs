fn main() {
    use std::collections::VecMap;
    
    let mut a = VecMap::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
}
