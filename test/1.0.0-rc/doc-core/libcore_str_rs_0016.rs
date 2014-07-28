fn main() {
    assert_eq!("11foo1bar11".trim_right_chars('1'), "11foo1bar")
    assert_eq!("12foo1bar12".trim_right_chars(&['1', '2']), "12foo1bar")
    assert_eq!("123foo1bar123".trim_right_chars(|c: char| c.is_digit()), "123foo1bar")
}
