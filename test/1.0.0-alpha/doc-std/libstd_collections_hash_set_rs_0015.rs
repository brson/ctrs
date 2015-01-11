fn main() {
    use std::collections::HashSet;
    
    let mut v = HashSet::new();
    assert_eq!(v.len(), 0);
    v.insert(1u);
    assert_eq!(v.len(), 1);
}
