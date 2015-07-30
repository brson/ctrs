fn main() {
    use std::cell::RefCell;
    use std::thread;
    
    let result = thread::spawn(move || {
    let c = RefCell::new(5);
    let m = c.borrow();
    
    let b = c.borrow_mut(); // this causes a panic
    }).join();
    
    assert!(result.is_err());
}
