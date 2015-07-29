fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};        let some_isize = AtomicIsize::new(5);        let value = some_isize.compare_and_swap(5, 10, Ordering::Relaxed);}
