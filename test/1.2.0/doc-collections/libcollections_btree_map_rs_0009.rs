fn main() {
    use std::collections::BTreeMap;
    
    let mut a = BTreeMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    
    let keys: Vec<_> = a.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
}
