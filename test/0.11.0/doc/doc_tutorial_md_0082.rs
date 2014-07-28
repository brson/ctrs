fn main() {
    struct Point { x: f64, y: f64 }
    let point = &box Point { x: 10.0, y: 20.0 };
    println!("{:f}", point.x);
}