fn main() {
    fn draw_circle(p: Point, f: f64) { }
    fn draw_rectangle(p: Point, p: Point) { }
    struct Point { x: f64, y: f64 }
    enum Shape {
        Circle(Point, f64),
        Rectangle(Point, Point)
    }
    impl Shape {
       fn draw_reference(&self) { /* ... */ }
       fn draw_owned(self: Box<Shape>) { /* ... */ }
       fn draw_value(self) { /* ... */ }
    }
    let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);
    // As with typical function arguments, owned pointers
    // are automatically converted to references
    
    (box s).draw_reference();
    
    // Unlike typical function arguments, the self value will
    // automatically be referenced ...
    s.draw_reference();
    
    // ... and dereferenced
    (& &s).draw_reference();
    
    // ... and dereferenced and borrowed
    (&box s).draw_reference();
}