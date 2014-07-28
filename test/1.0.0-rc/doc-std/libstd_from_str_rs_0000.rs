fn main() {
    assert_eq!(from_str::<bool>("true"), Some(true));
    assert_eq!(from_str::<bool>("false"), Some(false));
    assert_eq!(from_str::<bool>("not even a boolean"), None);
}
