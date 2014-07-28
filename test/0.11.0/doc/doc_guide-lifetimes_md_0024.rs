fn main() {
    struct Point {x: f64, y: f64}; // as before
    struct Size {w: f64, h: f64}; // as before
    enum Shape {
        Circle(Point, f64),   // origin, radius
        Rectangle(Point, Size)  // upper-left, dimensions
    }
    fn compute_area(shape: &Shape) -> f64 { 0.0 }
    fn select<'r, T>(shape: &Shape, threshold: f64,
                     a: &'r T, b: &'r T) -> &'r T {
        if compute_area(shape) > threshold {a} else {b}
    }
}