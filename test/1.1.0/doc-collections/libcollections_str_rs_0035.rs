fn main() {
    assert_eq!("123foo1bar123".trim_left_matches(|c: char| c.is_numeric()), "foo1bar123");}
