fn main() {
    use std::collections::DList;
    
    let mut d = DList::new();
    d.push_back(1i);
    d.push_back(3);
    assert_eq!(3, *d.back().unwrap());
}
