fn main() {
    use std::collections::BTreeMap;
    
    let mut a = BTreeMap::new();
    assert!(a.is_empty());
    a.insert(1u, "a");
    assert!(!a.is_empty());
}
