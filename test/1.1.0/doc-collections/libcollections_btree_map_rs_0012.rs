fn main() {
    use std::collections::BTreeMap;        let mut a = BTreeMap::new();    assert!(a.is_empty());    a.insert(1, "a");    assert!(!a.is_empty());}
