fn main() {
    use std::cmp::Ordering;
    
    let result = 1.0 < 2.0;
    assert_eq!(result, true);
    
    let result = 2.0 < 1.0;
    assert_eq!(result, false);
}
