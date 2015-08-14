fn main() {
    use std::fmt;
    
    struct Length(i32);
    
    impl fmt::Binary for Length {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let val = self.0;
    
            write!(f, "{:b}", val) // delegate to i32's implementation
        }
    }
    
    let l = Length(107);
    
    println!("l as binary is: {:b}", l);
}
