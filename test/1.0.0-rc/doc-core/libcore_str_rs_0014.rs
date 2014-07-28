fn main() {
    assert_eq!("11foo1bar11".trim_chars('1'), "foo1bar")
    assert_eq!("12foo1bar12".trim_chars(&['1', '2']), "foo1bar")
    assert_eq!("123foo1bar123".trim_chars(|c: char| c.is_digit()), "foo1bar")
}
