fn main() {
    use std::collections::RingBuf;
    
    let mut buf = RingBuf::new();
    buf.push_back(5i);
    buf.push_back(10i);
    buf.push_back(12i);
    buf.push_back(15);
    buf.remove(2);
    assert_eq!(Some(&15), buf.get(2));
}
