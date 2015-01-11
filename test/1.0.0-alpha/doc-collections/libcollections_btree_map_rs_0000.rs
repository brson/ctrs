fn main() {
    use std::collections::BTreeMap;
    
    let mut a = BTreeMap::new();
    a.insert(1u, "a");
    a.clear();
    assert!(a.is_empty());
}
