fn main() {
    use std::collections::BitvSet;
    
    let mut s = BitvSet::new();
    s.insert(32183231);
    s.remove(&32183231);
    
    // Internal storage will probably be bigger than necessary
    println!("old capacity: {}", s.capacity());
    
    // Now should be smaller
    s.shrink_to_fit();
    println!("new capacity: {}", s.capacity());
}
