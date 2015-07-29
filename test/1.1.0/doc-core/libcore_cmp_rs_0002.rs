fn main() {
    use std::cmp::Ordering;
    
    let mut data: &mut [_] = &mut [2, 10, 5, 8];
    
    // sort the array from largest to smallest.
    data.sort_by(|a, b| a.cmp(b).reverse());
    
    let b: &mut [_] = &mut [10, 8, 5, 2];
    assert!(data == b);
}
