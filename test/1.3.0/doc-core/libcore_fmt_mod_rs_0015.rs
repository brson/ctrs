fn main() {
    use std::fmt;
    
    struct Length(i32);
    
    impl fmt::Pointer for Length {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // use `as` to convert to a `*const T`, which implements Pointer, which we can use
    
            write!(f, "{:p}", self as *const Length)
        }
    }
    
    let l = Length(42);
    
    println!("l is in memory here: {:p}", l);
}
