fn main() {
    use std::collections::HashSet;
    
    let mut set = HashSet::new();
    
    set.insert(2u);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
}
