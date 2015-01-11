fn main() {
    let mut vec = vec![1i, 2i, 3i];
    vec.push(4);
    assert_eq!(vec, vec![1, 2, 3, 4]);
}
