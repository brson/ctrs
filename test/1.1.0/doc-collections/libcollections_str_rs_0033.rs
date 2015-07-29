fn main() {
    assert_eq!("123foo1bar123".trim_matches(|c: char| c.is_numeric()), "foo1bar");
}
