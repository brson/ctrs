fn main() {
    use std::collections::LinkedList;        let mut dl = LinkedList::new();    assert_eq!(dl.back(), None);        dl.push_back(1);    assert_eq!(dl.back(), Some(&1));    }
