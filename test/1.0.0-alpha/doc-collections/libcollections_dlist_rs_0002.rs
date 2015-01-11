fn main() {
    use std::collections::DList;
    
    let mut d = DList::new();
    assert_eq!(d.pop_back(), None);
    d.push_back(1i);
    d.push_back(3);
    assert_eq!(d.pop_back(), Some(3));
}
