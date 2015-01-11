fn main() {
    fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 { f(x) + f(x) }

    let square = |&: x: i32| { x * x };

    twice(5, square); // evaluates to 50

}
