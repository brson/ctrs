fn main() {
    use std::collections::RingBuf;
    
    let mut v = RingBuf::new();
    v.push_back(1i);
    v.clear();
    assert!(v.is_empty());
}
