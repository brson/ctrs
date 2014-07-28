fn main() {
    struct Point { x: f64, y: f64 }
    struct TuplePoint(f64, f64);
    mod game { pub struct User<'a> { pub name: &'a str, pub age: uint, pub score: uint } }
    struct Cookie; fn some_fn<T>(t: T) {}
    Point {x: 10.0, y: 20.0};
    TuplePoint(10.0, 20.0);
    let u = game::User {name: "Joe", age: 35, score: 100_000};
    some_fn::<Cookie>(Cookie);
}