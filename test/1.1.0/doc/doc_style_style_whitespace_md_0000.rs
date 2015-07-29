fn main() {
    #[deprecated = "Use `bar` instead."]
    fn foo(a: uint, b: uint) -> uint {
        a + b
    }
}
