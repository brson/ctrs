fn main() {
    let mut s = String::from("foo");
    s.reserve(100);
    assert!(s.capacity() >= 100);
    s.shrink_to_fit();
    assert_eq!(s.capacity(), 3);
}
