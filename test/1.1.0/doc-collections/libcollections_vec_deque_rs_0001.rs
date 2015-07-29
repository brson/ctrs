fn main() {
    use std::collections::VecDeque;
    
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    if let Some(elem) = buf.get_mut(1) {
        *elem = 7;
    }
    
    assert_eq!(buf[1], 7);
}
