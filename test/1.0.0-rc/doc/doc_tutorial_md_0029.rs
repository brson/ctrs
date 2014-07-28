fn main() {
    struct Point { x: f64, y: f64 }
    let mypoint = Point { x: 0.0, y: 0.0 };
    match mypoint {
        Point { x, .. } => println!("{}", x)
    }
}