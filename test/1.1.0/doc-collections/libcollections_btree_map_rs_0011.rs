fn main() {
    use std::collections::BTreeMap;        let mut a = BTreeMap::new();    assert_eq!(a.len(), 0);    a.insert(1, "a");    assert_eq!(a.len(), 1);}
