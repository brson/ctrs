fn main() {
    let mut vec = vec![1i, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, vec![1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, vec![1, 4, 2, 3, 5]);
}
