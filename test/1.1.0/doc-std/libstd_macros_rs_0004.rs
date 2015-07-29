fn main() {
    use std::fmt;        let s = fmt::format(format_args!("hello {}", "world"));    assert_eq!(s, format!("hello {}", "world"));    }
