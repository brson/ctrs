fn main() {
    struct Point {x: f64, y: f64}
    let on_the_stack : Point      =     Point {x: 3.0, y: 4.0};
    let on_the_heap  : Box<Point> = box Point {x: 7.0, y: 9.0};
}