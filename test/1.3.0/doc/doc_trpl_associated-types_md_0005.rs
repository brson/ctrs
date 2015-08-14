fn main() {
    use std::fmt;
    
    trait Graph {
        type N: fmt::Display;
        type E;
    
        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
    }
}
