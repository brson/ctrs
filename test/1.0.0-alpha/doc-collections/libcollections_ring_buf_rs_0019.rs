fn main() {
    use std::collections::RingBuf;
    
    let mut d = RingBuf::new();
    d.push_front(1i);
    d.push_front(2i);
    assert_eq!(d.front(), Some(&2i));
}
