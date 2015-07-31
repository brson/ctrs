fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let origin = Point { x: 0, y: 0 };
    
    println!("The origin is: {:?}", origin);
}
