fn main() {
    let mut vec = vec![1i, 2, 3];
    assert!(vec.shift() == Some(1));
    assert_eq!(vec, vec![2, 3]);
}
