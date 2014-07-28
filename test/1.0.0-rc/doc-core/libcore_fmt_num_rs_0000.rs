fn main() {
    use std::fmt::radix;
    assert_eq!(format!("{}", radix(55i, 36)), "1j".to_string());
}
