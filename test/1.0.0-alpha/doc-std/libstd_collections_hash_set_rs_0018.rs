fn main() {
    use std::collections::HashSet;
    
    let set: HashSet<uint> = [1, 2, 3].iter().map(|&x| x).collect();
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&4), false);
}
