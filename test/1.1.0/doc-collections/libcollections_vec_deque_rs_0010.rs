fn main() {
    use std::collections::VecDeque;
    
    let mut v = VecDeque::new();
    assert_eq!(v.len(), 0);
    v.push_back(1);
    assert_eq!(v.len(), 1);
}
