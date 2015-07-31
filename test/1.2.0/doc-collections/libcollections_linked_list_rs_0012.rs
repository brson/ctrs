fn main() {
    use std::collections::LinkedList;
    
    let mut d = LinkedList::new();
    
    d.push_front(1);
    d.push_front(2);
    d.push_front(3);
    
    let mut splitted = d.split_off(2);
    
    assert_eq!(splitted.pop_front(), Some(1));
    assert_eq!(splitted.pop_front(), None);
}
