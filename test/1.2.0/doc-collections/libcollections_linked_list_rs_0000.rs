fn main() {
    use std::collections::LinkedList;
    
    let mut a = LinkedList::new();
    let mut b = LinkedList::new();
    a.push_back(1);
    a.push_back(2);
    b.push_back(3);
    b.push_back(4);
    
    a.append(&mut b);
    
    for e in &a {
    println!("{}", e); // prints 1, then 2, then 3, then 4
    }
    println!("{}", b.len()); // prints 0
}
