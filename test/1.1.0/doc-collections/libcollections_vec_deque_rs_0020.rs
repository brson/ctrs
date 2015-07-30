fn main() {
    use std::collections::VecDeque;
    
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(3);
    assert_eq!(3, *buf.back().unwrap());
}
