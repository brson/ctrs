fn main() {
    use std::cell::RefCell;
    
    let c = RefCell::new(5);
    
    let borrowed_five = c.borrow_mut();
}
