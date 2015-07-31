fn main() {
    let mut s = String::from("hello");
    s.truncate(2);
    assert_eq!(s, "he");
}
