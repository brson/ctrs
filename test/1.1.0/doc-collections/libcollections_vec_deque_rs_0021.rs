fn main() {
    use std::collections::VecDeque;        let mut buf = VecDeque::new();    assert_eq!(buf.pop_back(), None);    buf.push_back(1);    buf.push_back(3);    assert_eq!(buf.pop_back(), Some(3));}
