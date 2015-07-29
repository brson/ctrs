fn main() {
    use std::collections::HashSet;
    
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut b = HashSet::new();
    
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);
}
