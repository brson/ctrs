fn main() {
    struct Point {x: f64, y: f64}; // as before
    struct Size {w: f64, h: f64}; // as before
    enum Shape {
        Circle(Point, f64),   // origin, radius
        Rectangle(Point, Size)  // upper-left, dimensions
    }
    static tau: f64 = 6.28;
    fn compute_area(shape: &Shape) -> f64 {
        match *shape {
            Circle(_, radius) => 0.5 * tau * radius * radius,
            Rectangle(_, ref size) => size.w * size.h
        }
    }
}