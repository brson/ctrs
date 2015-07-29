fn main() {
    use std::collections::HashSet;        let mut v = HashSet::new();    v.insert(1);    v.clear();    assert!(v.is_empty());}
