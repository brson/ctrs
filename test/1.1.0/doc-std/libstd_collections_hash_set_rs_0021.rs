fn main() {
    use std::collections::HashSet;
    
    let sub: HashSet<_> = [1, 2].iter().cloned().collect();
    let mut set = HashSet::new();
    
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(0);
    set.insert(1);
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(2);
    assert_eq!(set.is_superset(&sub), true);
}
