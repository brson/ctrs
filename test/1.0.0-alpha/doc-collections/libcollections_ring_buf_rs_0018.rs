fn main() {
    use std::collections::RingBuf;
    
    let mut d = RingBuf::new();
    d.push_back(1i);
    d.push_back(2i);
    
    assert_eq!(d.pop_front(), Some(1i));
    assert_eq!(d.pop_front(), Some(2i));
    assert_eq!(d.pop_front(), None);
}
