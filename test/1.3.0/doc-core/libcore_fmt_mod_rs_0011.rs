fn main() {
    use std::fmt;
    
    struct Length(i32);
    
    impl fmt::LowerHex for Length {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let val = self.0;
    
            write!(f, "{:x}", val) // delegate to i32's implementation
        }
    }
    
    let l = Length(9);
    
    println!("l as hex is: {:x}", l);
}
