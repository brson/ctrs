fn main() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    
    // Will print in an arbitrary order.
    for x in set.iter() {
        println!("{}", x);
    }
}
