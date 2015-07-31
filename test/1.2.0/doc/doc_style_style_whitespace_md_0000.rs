fn main() {
    #[deprecated = "Use `bar` instead."]
    fn foo(a: usize, b: usize) -> usize {
    a + b
    }
}
