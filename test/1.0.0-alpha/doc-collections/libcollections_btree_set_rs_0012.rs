fn main() {
    use std::collections::BTreeSet;
    
    let sup: BTreeSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    let mut set: BTreeSet<int> = BTreeSet::new();
    
    assert_eq!(set.is_subset(&sup), true);
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);
}
