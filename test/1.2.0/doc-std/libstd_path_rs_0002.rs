fn main() {
    use std::path;
    
    assert!(path::is_separator('/'));
    assert!(!path::is_separator('❤'));
}
