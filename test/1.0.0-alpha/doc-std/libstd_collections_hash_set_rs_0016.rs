fn main() {
    use std::collections::HashSet;
    
    let mut v = HashSet::new();
    assert!(v.is_empty());
    v.insert(1u);
    assert!(!v.is_empty());
}
