fn main() {
    use std::boxed::Box;

    let x: Box<int> = Box::new(10);

    let y = x;

    // attempting to use `x` will result in an error here

}
