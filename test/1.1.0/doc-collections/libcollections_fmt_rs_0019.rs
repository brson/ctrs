fn main() {
    use std::fmt;
    
    let s = fmt::format(format_args!("Hello, {}!", "world"));
    assert_eq!(s, "Hello, world!".to_string());
}
