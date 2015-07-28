fn main() {
    assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
}
