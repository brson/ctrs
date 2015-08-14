fn main() {
    use std::marker;
    
    struct Vec<T> {
        data: *const T, // *const for covariance!
        len: usize,
        cap: usize,
        _marker: marker::PhantomData<T>,
    }
}
