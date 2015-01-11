fn main() {
    use std::collections::RingBuf;
    
    let mut buf = RingBuf::new();
    buf.push_back(3i);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(1).unwrap(), &4);
}
