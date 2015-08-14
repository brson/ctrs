fn main() {
    use std::cell::Cell;
    
    struct Point {
        x: i32,
        y: Cell<i32>,
    }
    
    let point = Point { x: 5, y: Cell::new(6) };
    
    point.y.set(7);
    
    println!("y: {:?}", point.y);
}
