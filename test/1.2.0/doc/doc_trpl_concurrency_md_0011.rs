fn main() {
    use std::thread;
    
    let result = thread::spawn(move || {
    panic!("oops!");
    }).join();
    
    assert!(result.is_err());
}
