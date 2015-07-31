fn main() {
    use std::cell::UnsafeCell;
    use std::marker::Sync;
    
    struct NotThreadSafe<T> {
    value: UnsafeCell<T>,
    }
    
    unsafe impl<T> Sync for NotThreadSafe<T> {}
}
