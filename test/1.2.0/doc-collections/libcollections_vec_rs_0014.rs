fn main() {
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec.capacity(), 10);
    vec.shrink_to_fit();
    assert!(vec.capacity() >= 3);
}
