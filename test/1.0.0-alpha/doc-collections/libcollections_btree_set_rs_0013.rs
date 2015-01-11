fn main() {
    use std::collections::BTreeSet;
    
    let sub: BTreeSet<int> = [1i, 2].iter().map(|&x| x).collect();
    let mut set: BTreeSet<int> = BTreeSet::new();
    
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(0);
    set.insert(1);
    assert_eq!(set.is_superset(&sub), false);
    
    set.insert(2);
    assert_eq!(set.is_superset(&sub), true);
}
