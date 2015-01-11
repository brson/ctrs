fn main() {
    use std::collections::BTreeSet;
    
    let a: BTreeSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    let mut b: BTreeSet<int> = BTreeSet::new();
    
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);
}
