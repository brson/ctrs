fn main() {
    use std::collections::RingBuf;
    
    let mut v = RingBuf::new();
    assert_eq!(v.len(), 0);
    v.push_back(1i);
    assert_eq!(v.len(), 1);
}
