fn main() {
    use std::collections::BTreeSet;
    
    let mut v = BTreeSet::new();
    assert_eq!(v.len(), 0);
    v.insert(1);
    assert_eq!(v.len(), 1);
}
