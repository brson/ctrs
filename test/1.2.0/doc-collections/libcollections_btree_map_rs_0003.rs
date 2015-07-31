fn main() {
    use std::collections::BTreeMap;
    
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
    *x = "b";
    }
    assert_eq!(map[&1], "b");
}
