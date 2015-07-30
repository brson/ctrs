fn main() {
    enum Kind {
    A,
    B,
    C,
    }
    
    impl Default for Kind {
    fn default() -> Kind { Kind::A }
    }
}
