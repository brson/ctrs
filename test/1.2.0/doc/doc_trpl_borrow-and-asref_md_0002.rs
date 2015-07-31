fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("Foo".to_string(), 42);
    
    assert_eq!(map.get("Foo"), Some(&42));
}
