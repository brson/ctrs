fn main() {
    use std::cell::RefCell;
    use std::thread;
    
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    
    FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    *f.borrow_mut() = 2;
    });
    
    // each thread starts out with the initial value of 1
    thread::spawn(move|| {
    FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    *f.borrow_mut() = 3;
    });
    });
    
    // we retain our original value of 2 despite the child thread
    FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
    });
}
