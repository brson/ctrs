fn main() {
    use std::collections::VecDeque;
    
    let mut d = VecDeque::new();
    d.push_front(1);
    d.push_front(2);
    assert_eq!(d.front(), Some(&2));
}
