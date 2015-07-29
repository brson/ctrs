fn main() {
    use std::collections::BTreeSet;        let mut v = BTreeSet::new();    v.insert(1);    v.clear();    assert!(v.is_empty());}
