fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};        let some_isize = AtomicIsize::new(5);        let value = some_isize.load(Ordering::Relaxed);}
