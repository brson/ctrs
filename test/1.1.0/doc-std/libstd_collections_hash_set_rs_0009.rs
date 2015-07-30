fn main() {
    use std::collections::HashSet;
    
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 2);
}
