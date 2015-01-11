fn main() {
    use std::collections::RingBuf;
    
    let mut buf = RingBuf::new();
    buf.push_back(1i);
    buf.push_back(3);
    assert_eq!(3, *buf.back().unwrap());
}
