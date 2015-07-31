fn main() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = lock.clone();
    
    let _ = thread::spawn(move || -> () {
        // This thread will acquire the mutex first, unwrapping the result of
        // `lock` because the lock has not been poisoned.
        let _lock = lock2.lock().unwrap();
    
        // This panic while holding the lock (`_guard` is in scope) will poison
        // the mutex.
        panic!();
    }).join();
    
    // The lock is poisoned by this point, but the returned result can be
    // pattern matched on to return the underlying guard on both branches.
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    
    *guard += 1;
}
