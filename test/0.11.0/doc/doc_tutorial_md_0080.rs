fn main() {
    struct Point { x: f64, y: f64 }
    enum Shape { Rectangle(Point, Point) }
    impl Shape { fn area(&self) -> int { 0 } }
    let start = box Point { x: 10.0, y: 20.0 };
    let end = box Point { x: (*start).x + 100.0, y: (*start).y + 100.0 };
    let rect = &Rectangle(*start, *end);
    let area = (*rect).area();
}