fn main() {
    use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};        static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;        let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);    println!("live threads: {}", old_thread_count + 1);}
