fn main() {
    // In a trait, `self` refers to the self argument.
    // `Self` refers to the type implementing the trait.
    trait PartialEq {
        fn equals(&self, other: &Self) -> bool;
    }
    
    // In an impl, `self` refers just to the value of the receiver
    impl PartialEq for int {
        fn equals(&self, other: &int) -> bool { *other == *self }
    }
}