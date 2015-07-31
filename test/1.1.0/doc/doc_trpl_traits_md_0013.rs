fn main() {
    fn foo<T: Clone>(x: T) {
        x.clone();
    }
}
