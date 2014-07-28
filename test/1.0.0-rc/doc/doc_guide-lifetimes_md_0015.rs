fn main() {
    struct Point {x: f64, y: f64}; // as before
    struct Size {w: f64, h: f64}; // as before
    enum Shape {
        Circle(Point, f64),   // origin, radius
        Rectangle(Point, Size)  // upper-left, dimensions
    }
}