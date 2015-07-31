fn main() {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    
    let v: Vec<_> = set.into_iter().collect();
    assert_eq!(v, [1, 2, 3, 4]);
}
