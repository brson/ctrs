fn main() {
    let mut s = String::new();
    s.reserve_exact(10);
    assert!(s.capacity() >= 10);
}
