fn main() {
    assert_eq!("true".parse(), Ok(true));
    assert_eq!("false".parse(), Ok(false));
    assert!("not even a boolean".parse::<bool>().is_err());
}
