fn main() {
    use std::collections::HashSet;        let set: HashSet<_> = [1, 2, 3].iter().cloned().collect();    assert_eq!(set.contains(&1), true);    assert_eq!(set.contains(&4), false);}
