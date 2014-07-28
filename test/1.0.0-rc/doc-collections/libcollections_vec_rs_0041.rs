fn main() {
    let mut vec = vec![1i, 2, 3, 4];
    assert!(vec.mut_slice(0, 2) == [1, 2]);
}
