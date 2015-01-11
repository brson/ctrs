fn main() {
    let mut vec: Vec<int> = vec![1];
    vec.reserve(10);
    assert!(vec.capacity() >= 11);
}
