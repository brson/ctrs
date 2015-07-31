fn main() {
    let s = String::from("hello");
    let bytes = s.into_bytes();
    assert_eq!(bytes, [104, 101, 108, 108, 111]);
}
