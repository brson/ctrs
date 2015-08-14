fn main() {
    trait Foo {
        fn is_valid(&self) -> bool;
    
        fn is_invalid(&self) -> bool { !self.is_valid() }
    }
}
