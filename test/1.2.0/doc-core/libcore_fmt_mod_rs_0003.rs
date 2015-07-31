fn main() {
    use std::fmt;
    
    struct Foo(i32, String);
    
    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_tuple("Foo")
                .field(&self.0)
                .field(&self.1)
                .finish()
        }
    }
    
    // prints "Foo(10, "Hello World")"
    println!("{:?}", Foo(10, "Hello World".to_string()));
}
