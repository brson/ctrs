fn main() {
    use std::fmt;
    
    struct Length(i32);
    
    impl fmt::UpperExp for Length {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let val = self.0;
            write!(f, "{}E1", val / 10)
        }
    }
    
    let l = Length(100);
    
    println!("l in scientific notation is: {:E}", l);
}
