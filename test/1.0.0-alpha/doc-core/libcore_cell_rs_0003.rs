fn main() {
    use std::cell::UnsafeCell;
    use std::marker;
    
    struct NotThreadSafe<T> {
        value: UnsafeCell<T>,
        marker: marker::NoSync
    }
}
