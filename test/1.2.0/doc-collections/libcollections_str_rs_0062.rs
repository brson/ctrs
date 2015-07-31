fn main() {
    assert_eq!("1fooX".trim_left_matches(|c| c == '1' || c == 'X'), "fooX");
}
