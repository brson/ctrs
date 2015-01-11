fn main() {
    use std::collections::RingBuf;
    
    let mut d = RingBuf::new();
    assert_eq!(d.back(), None);
    
    d.push_back(1i);
    d.push_back(2i);
    assert_eq!(d.back(), Some(&2i));
}
