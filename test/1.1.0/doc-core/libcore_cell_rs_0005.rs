fn main() {
    use std::cell::Cell;
    
    let c = Cell::new(5);
    
    c.set(10);
}
