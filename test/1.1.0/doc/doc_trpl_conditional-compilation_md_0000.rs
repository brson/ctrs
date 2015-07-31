fn main() {
    #[cfg(foo)]
    fn foo() {}
    
    #[cfg(bar = "baz")]
    fn bar() {}
}
