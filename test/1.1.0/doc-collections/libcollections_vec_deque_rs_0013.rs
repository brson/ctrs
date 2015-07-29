fn main() {
    use std::collections::VecDeque;
    
    let mut v = VecDeque::new();
    v.push_back(1);
    v.clear();
    assert!(v.is_empty());
}
