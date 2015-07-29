fn main() {
    use std::collections::LinkedList;        let mut d = LinkedList::new();    assert_eq!(d.pop_front(), None);        d.push_front(1);    d.push_front(3);    assert_eq!(d.pop_front(), Some(3));    assert_eq!(d.pop_front(), Some(1));    assert_eq!(d.pop_front(), None);    }
