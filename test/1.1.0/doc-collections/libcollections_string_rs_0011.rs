fn main() {
    let mut s = String::new();
    s.reserve(10);
    assert!(s.capacity() >= 10);
}
