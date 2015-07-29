fn main() {
    use std::fmt;
    struct Foo; // our custom type
    impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "testing, testing")
    } }
}
