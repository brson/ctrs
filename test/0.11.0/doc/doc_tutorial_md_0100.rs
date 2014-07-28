fn main() {
    fn draw_circle(p: Point, f: f64) { }
    fn draw_rectangle(p: Point, p: Point) { }
    struct Point {
        x: f64,
        y: f64
    }
    
    enum Shape {
        Circle(Point, f64),
        Rectangle(Point, Point)
    }
    
    impl Shape {
        fn draw(&self) {
            match *self {
                Circle(p, f) => draw_circle(p, f),
                Rectangle(p1, p2) => draw_rectangle(p1, p2)
            }
        }
    }
    
    let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);
    s.draw();
}