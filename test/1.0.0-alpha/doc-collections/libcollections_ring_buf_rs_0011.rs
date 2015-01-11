fn main() {
    use std::collections::RingBuf;
    
    let mut v = RingBuf::new();
    assert!(v.is_empty());
    v.push_front(1i);
    assert!(!v.is_empty());
}
