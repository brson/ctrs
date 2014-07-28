fn main() {
    struct Point {x: f64, y: f64}
    fn get_x<'r>(p: &'r Point) -> &'r f64 { &p.x }
}