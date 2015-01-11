fn main() {
    let mut vec = vec![1i, 2, 2, 3, 2];
    
    vec.dedup();
    
    assert_eq!(vec, vec![1i, 2, 3, 2]);
}
