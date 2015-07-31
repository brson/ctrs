fn main() {
    let s = String::from("hello");
    assert_eq!(s.as_bytes(), [104, 101, 108, 108, 111]);
}
