fn main() {
    assert_eq!("123foo1bar123".trim_right_matches(|c: char| c.is_numeric()), "123foo1bar");
}
