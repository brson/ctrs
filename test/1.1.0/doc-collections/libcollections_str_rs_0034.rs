fn main() {
    assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");        let x: &[_] = &['1', '2'];    assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");}
