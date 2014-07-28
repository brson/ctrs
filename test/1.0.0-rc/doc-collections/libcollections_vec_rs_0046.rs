fn main() {
    let vec = vec![1i, 2, 3];
    assert!(vec.slice_from(1) == [2, 3]);
}
