fn main() {
    use std::cell::RefCell;

    

    let x = RefCell::new(42);

    

    let y = x.borrow_mut();

}
