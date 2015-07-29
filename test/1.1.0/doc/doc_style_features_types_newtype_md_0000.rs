fn main() {
    struct Miles(pub f64);
    struct Kilometers(pub f64);
    
    impl Miles {
        fn as_kilometers(&self) -> Kilometers { ... }
    }
    impl Kilometers {
        fn as_miles(&self) -> Miles { ... }
    }
}
