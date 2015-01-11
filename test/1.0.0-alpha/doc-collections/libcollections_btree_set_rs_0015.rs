fn main() {
    use std::collections::BTreeSet;
    
    let mut set = BTreeSet::new();
    
    set.insert(2i);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
}
