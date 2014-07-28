fn main() {
    use std::fmt;
    
    let s = format_args!(fmt::format, "Hello, {}!", "world");
    assert_eq!(s, "Hello, world!".to_string());
}
