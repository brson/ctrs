fn main() {
    use std::collections::RingBuf;
    
    let mut buf = RingBuf::new();
    assert_eq!(buf.pop_back(), None);
    buf.push_back(1i);
    buf.push_back(3);
    assert_eq!(buf.pop_back(), Some(3));
}
