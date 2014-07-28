fn main() {
    let mut v = vec![1i, 2, 3];
    assert_eq!(v.remove(1), Some(2));
    assert_eq!(v, vec![1, 3]);
    
    assert_eq!(v.remove(4), None);
    // v is unchanged:
    assert_eq!(v, vec![1, 3]);
}
