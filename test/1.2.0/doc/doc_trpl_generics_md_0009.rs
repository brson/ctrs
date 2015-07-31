fn main() {
    struct Point<T> {
    x: T,
    y: T,
    }
    
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
}
