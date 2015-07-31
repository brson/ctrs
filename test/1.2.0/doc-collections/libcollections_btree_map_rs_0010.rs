fn main() {
    use std::collections::BTreeMap;
    
    let mut a = BTreeMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    
    let values: Vec<&str> = a.values().cloned().collect();
    assert_eq!(values, ["a", "b"]);
}
