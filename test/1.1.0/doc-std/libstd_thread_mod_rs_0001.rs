fn main() {
    use std::thread;
    
    let child = thread::spawn(move || {
        // some work here
    });
    // some work here
    let res = child.join();
}
