fn main() {
    struct Point { x: f64, y: f64 }
    enum Shape {
        Circle(Point, f64),
        Rectangle(Point, Point)
    }
}