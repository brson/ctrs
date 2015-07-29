fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};        let some_bool = AtomicBool::new(true);        let value = some_bool.swap(false, Ordering::Relaxed);}
