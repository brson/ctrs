fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {

    f(x) + f(x)

}



fn main() {

    twice(5, |x: i32| { x * x }); // evaluates to 50

}

