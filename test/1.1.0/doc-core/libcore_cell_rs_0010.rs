fn main() {
    use std::cell::RefCell;
    use std::thread;
    
    let result = thread::spawn(move || {
       let c = RefCell::new(5);
       let m = c.borrow_mut();
    
       let b = c.borrow(); // this causes a panic
    }).join();
    
    assert!(result.is_err());
}
