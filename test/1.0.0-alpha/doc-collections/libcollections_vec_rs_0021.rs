fn main() {
    let mut vec = vec![1i, 2, 3, 4];
    vec.retain(|&x| x%2 == 0);
    assert_eq!(vec, vec![2, 4]);
}
