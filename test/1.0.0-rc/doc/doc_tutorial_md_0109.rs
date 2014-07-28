fn main() {
    // This does
    fn head<T: Clone>(v: &[T]) -> T {
        v[0].clone()
    }
}