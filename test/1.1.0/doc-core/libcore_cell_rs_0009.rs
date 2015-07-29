fn main() {
    use std::cell::RefCell;
    
    let c = RefCell::new(5);
    
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
}
