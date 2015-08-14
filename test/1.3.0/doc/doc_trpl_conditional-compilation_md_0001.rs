fn main() {
    #[cfg(any(unix, windows))]
    fn foo() {}
    
    #[cfg(all(unix, target_pointer_width = "32"))]
    fn bar() {}
    
    #[cfg(not(foo))]
    fn not_foo() {}
}
