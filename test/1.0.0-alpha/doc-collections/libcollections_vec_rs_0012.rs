fn main() {
    let mut vec: Vec<int> = vec![1];
    vec.reserve_exact(10);
    assert!(vec.capacity() >= 11);
}
