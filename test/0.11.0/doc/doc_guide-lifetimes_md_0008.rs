fn main() {
    struct Point {x: f64, y: f64}
    struct Size {w: f64, h: f64} // as before
    struct Rectangle {origin: Point, size: Size}
    let rect_stack   =    &Rectangle {origin: Point {x: 1.0, y: 2.0},
                                      size: Size {w: 3.0, h: 4.0}};
    let rect_heap    = box Rectangle {origin: Point {x: 5.0, y: 6.0},
                                      size: Size {w: 3.0, h: 4.0}};
}