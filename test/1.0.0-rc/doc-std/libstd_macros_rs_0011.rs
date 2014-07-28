fn main() {
    use std::fmt;
    
    let s = format_args!(fmt::format, "hello {}", "world");
    assert_eq!(s, format!("hello {}", "world"));
    
    format_args!(|args| {
        // pass `args` to another function, etc.
    }, "hello {}", "world");
}
