fn main() {
    use std::fmt;
    
    struct Point {
    x: i32,
    y: i32,
    }
    
    impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
    }
    }
    
    let origin = Point { x: 0, y: 0 };
    
    println!("The origin is: {:?}", origin);
}
