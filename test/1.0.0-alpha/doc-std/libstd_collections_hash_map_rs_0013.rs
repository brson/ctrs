fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("a", 1i);
    map.insert("b", 2);
    map.insert("c", 3);
    
    // Not possible with .iter()
    let vec: Vec<(&str, int)> = map.into_iter().collect();
}
