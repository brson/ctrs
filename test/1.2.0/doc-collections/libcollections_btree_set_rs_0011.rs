fn main() {
    use std::collections::BTreeSet;
    
    let a: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut b = BTreeSet::new();
    
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);
}
