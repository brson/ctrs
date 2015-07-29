fn main() {
    use std::collections::VecDeque;        let mut buf = VecDeque::new();    buf.push_back(5);    buf.push_back(10);    buf.push_back(12);    buf.push_back(15);    buf.remove(2);    assert_eq!(Some(&15), buf.get(2));}
