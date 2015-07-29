fn main() {
    use std::collections::LinkedList;
    
    let mut dl = LinkedList::new();
    
    dl.push_front(2);
    dl.push_front(1);
    assert_eq!(dl.len(), 2);
    assert_eq!(dl.front(), Some(&1));
    
    dl.clear();
    assert_eq!(dl.len(), 0);
    assert_eq!(dl.front(), None);
    
}
