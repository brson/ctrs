fn main() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let barrier = Arc::new(Barrier::new(10));
    for _ in 0..10 {
    let c = barrier.clone();
    // The same messages will be printed together.
    // You will NOT see any interleaving.
    thread::spawn(move|| {
    println!("before wait");
    c.wait();
    println!("after wait");
    });
    }
}
