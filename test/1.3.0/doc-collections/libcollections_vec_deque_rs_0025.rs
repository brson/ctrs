fn main() {
    use std::collections::VecDeque;
    
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    
    assert_eq!(buf.remove(1), Some(2));
    assert_eq!(buf.get(1), Some(&3));
}
