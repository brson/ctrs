use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
let spinlock = Arc::new(AtomicUsize::new(1));

let spinlock_clone = spinlock.clone();
thread::spawn(move|| {
spinlock_clone.store(0, Ordering::SeqCst);
});

// Wait for the other thread to release the lock
while spinlock.load(Ordering::SeqCst) != 0 {}
}
