fn main() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert("a".to_string());
    set.insert("b".to_string());
    
    // Not possible to collect to a Vec<String> with a regular `.iter()`.
    let v: Vec<String> = set.into_iter().collect();
    
    // Will print in an arbitrary order.
    for x in &v {
        println!("{}", x);
    }
}
