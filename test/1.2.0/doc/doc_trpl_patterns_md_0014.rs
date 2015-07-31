fn main() {
    struct Point {
        x: i32,
        y: i32,
    }
    
    let origin = Point { x: 0, y: 0 };
    
    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }
}
