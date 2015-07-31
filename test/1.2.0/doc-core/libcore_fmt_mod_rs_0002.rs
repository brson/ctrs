fn main() {
    use std::fmt;
    
    struct Foo {
        bar: i32,
        baz: String,
    }
    
    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("Foo")
                .field("bar", &self.bar)
                .field("baz", &self.baz)
                .finish()
        }
    }
    
    // prints "Foo { bar: 10, baz: "Hello World" }"
    println!("{:?}", Foo { bar: 10, baz: "Hello World".to_string() });
}
