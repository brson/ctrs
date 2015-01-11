fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("a", 1i);
    map.insert("b", 2);
    map.insert("c", 3);
    
    // Update all values
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    
    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
    }
}
