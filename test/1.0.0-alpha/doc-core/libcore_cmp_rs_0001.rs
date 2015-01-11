fn main() {
    use std::cmp::Ordering::{Less, Equal, Greater};
    
    assert_eq!(Less.reverse(), Greater);
    assert_eq!(Equal.reverse(), Equal);
    assert_eq!(Greater.reverse(), Less);
    
    let mut data: &mut [_] = &mut [2u, 10, 5, 8];
    
    // sort the array from largest to smallest.
    data.sort_by(|a, b| a.cmp(b).reverse());
    
    let b: &mut [_] = &mut [10u, 8, 5, 2];
    assert!(data == b);
}
