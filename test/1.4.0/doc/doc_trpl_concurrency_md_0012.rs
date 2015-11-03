fn main() {
    use std::thread;
    
    let handle = thread::spawn(move || {
        panic!("oops!");
    });
    
    let result = handle.join();
    
    assert!(result.is_err());
}
