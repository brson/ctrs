fn main() {
    use std::cell::RefCell;        let c = RefCell::new(5);        let five = c.into_inner();}
