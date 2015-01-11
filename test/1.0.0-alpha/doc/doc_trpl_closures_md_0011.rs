fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {

    f(x) + f(x)

}



fn square(x: i32) -> i32 { x * x }



fn main() {

    twice(5, square); // evaluates to 50

}

