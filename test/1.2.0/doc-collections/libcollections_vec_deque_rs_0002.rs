fn main() {
    use std::collections::VecDeque;
    
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    buf.swap(0, 2);
    assert_eq!(buf[0], 5);
    assert_eq!(buf[2], 3);
}
