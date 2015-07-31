fn main() {
    use std::fmt;
    
    struct Foo(Vec<i32>);
    
    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_list().entries(self.0.iter()).finish()
        }
    }
    
    // prints "[10, 11]"
    println!("{:?}", Foo(vec![10, 11]));
}
