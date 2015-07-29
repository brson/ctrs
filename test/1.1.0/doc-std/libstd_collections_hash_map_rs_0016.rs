fn main() {
    use std::collections::HashMap;
    
    let mut a = HashMap::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
}
