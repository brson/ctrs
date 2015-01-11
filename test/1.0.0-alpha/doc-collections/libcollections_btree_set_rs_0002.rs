fn main() {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<uint> = [1u, 2, 3, 4].iter().map(|&x| x).collect();
    
    let v: Vec<uint> = set.into_iter().collect();
    assert_eq!(v, vec![1u,2,3,4]);
}
