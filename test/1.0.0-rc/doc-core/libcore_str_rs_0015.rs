fn main() {
    assert_eq!("11foo1bar11".trim_left_chars('1'), "foo1bar11")
    assert_eq!("12foo1bar12".trim_left_chars(&['1', '2']), "foo1bar12")
    assert_eq!("123foo1bar123".trim_left_chars(|c: char| c.is_digit()), "foo1bar123")
}
