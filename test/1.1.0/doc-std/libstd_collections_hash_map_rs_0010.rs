fn main() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    for key in map.keys() {
        println!("{}", key);
    }
}
