fn main() {
    use std::collections::BTreeSet;
    
    let sup: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut set = BTreeSet::new();
    
    assert_eq!(set.is_subset(&sup), true);
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);
}
