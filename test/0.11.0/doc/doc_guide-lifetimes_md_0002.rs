fn main() {
    struct Point {x: f64, y: f64}
    fn sqrt(f: f64) -> f64 { 0.0 }
    fn compute_distance(p1: &Point, p2: &Point) -> f64 {
        let x_d = p1.x - p2.x;
        let y_d = p1.y - p2.y;
        sqrt(x_d * x_d + y_d * y_d)
    }
}