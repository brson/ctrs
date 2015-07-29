fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};        let some_isize = AtomicIsize::new(5);        some_isize.store(10, Ordering::Relaxed);}
