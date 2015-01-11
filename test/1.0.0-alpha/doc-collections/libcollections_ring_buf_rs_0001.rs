fn main() {
    use std::collections::RingBuf;
    
    let mut buf = RingBuf::new();
    buf.push_back(3i);
    buf.push_back(4);
    buf.push_back(5);
    match buf.get_mut(1) {
        None => {}
        Some(elem) => {
            *elem = 7;
        }
    }
    
    assert_eq!(buf[1], 7);
}
