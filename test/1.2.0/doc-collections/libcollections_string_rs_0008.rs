fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!(s, "foobar");
}
