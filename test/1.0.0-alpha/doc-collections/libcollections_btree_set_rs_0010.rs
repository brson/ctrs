fn main() {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&4), false);
}
