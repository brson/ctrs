fn main() {
    use std::collections::BTreeSet;
    
    let mut set = BTreeSet::new();
    
    assert_eq!(set.insert(2i), true);
    assert_eq!(set.insert(2i), false);
    assert_eq!(set.len(), 1);
}
