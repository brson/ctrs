fn main() {
    use std::fmt;
    
    struct Foo(Vec<(String, i32)>);
    
    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
        }
    }
    
    // prints "{"A": 10, "B": 11}"
    println!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)]));
}
