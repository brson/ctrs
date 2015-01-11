fn main() {
    use std::default::Default;
    
    enum Kind {
        A,
        B,
        C,
    }
    
    impl Default for Kind {
        fn default() -> Kind { Kind::A }
    }
}
