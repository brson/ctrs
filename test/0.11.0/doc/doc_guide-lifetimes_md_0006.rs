fn main() {
    struct Point {x: f64, y: f64}
    let tmp = Point {x: 3.0, y: 4.0};
    let on_the_stack2 : &Point = &tmp;
}