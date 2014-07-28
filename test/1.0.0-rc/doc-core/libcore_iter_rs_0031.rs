fn main() {
    use std::iter::{NoElements, OneElement, MinMax};
    
    let v: [int, ..0] = [];
    assert_eq!(v.iter().min_max(), NoElements);
    
    let v = [1i];
    assert!(v.iter().min_max() == OneElement(&1));
    
    let v = [1i, 2, 3, 4, 5];
    assert!(v.iter().min_max() == MinMax(&1, &5));
    
    let v = [1i, 2, 3, 4, 5, 6];
    assert!(v.iter().min_max() == MinMax(&1, &6));
    
    let v = [1i, 1, 1, 1];
    assert!(v.iter().min_max() == MinMax(&1, &1));
}
