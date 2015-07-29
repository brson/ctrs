fn main() {
    use std::collections::VecDeque;        let mut v = VecDeque::new();    assert!(v.is_empty());    v.push_front(1);    assert!(!v.is_empty());}
