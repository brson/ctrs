fn main() {
    let mut vec = vec![1, 2, 3, 4];
    vec.truncate(2);
    assert_eq!(vec, [1, 2]);
}
