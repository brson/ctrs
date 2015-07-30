fn main() {
    use std::collections::VecDeque;
    
    let mut d = VecDeque::new();
    assert_eq!(d.back(), None);
    
    d.push_back(1);
    d.push_back(2);
    match d.back_mut() {
    Some(x) => *x = 9,
    None => (),
    }
    assert_eq!(d.back(), Some(&9));
}
