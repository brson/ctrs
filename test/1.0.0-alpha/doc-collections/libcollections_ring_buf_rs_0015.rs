fn main() {
    use std::collections::RingBuf;
    
    let mut d = RingBuf::new();
    assert_eq!(d.front_mut(), None);
    
    d.push_back(1i);
    d.push_back(2i);
    match d.front_mut() {
        Some(x) => *x = 9i,
        None => (),
    }
    assert_eq!(d.front(), Some(&9i));
}
