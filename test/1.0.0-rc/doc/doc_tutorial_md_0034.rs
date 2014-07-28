fn main() {
    struct Point { x: f64, y: f64 }
    enum Shape { Circle(Point, f64), Rectangle(Point, Point) }
    let circle = Circle(Point { x: 0.0, y: 0.0 }, 10.0);
}