fn main() {
    trait Foo {}
    trait Bar<T> {}
    
    fn sendable_foo(f: Box<Foo + Send>) { /* ... */ }
    fn shareable_bar<T: Share>(b: &Bar<T> + Share) { /* ... */ }
}