fn main() {
    let mut vec = vec![1i, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, vec![1, 2]);
}
