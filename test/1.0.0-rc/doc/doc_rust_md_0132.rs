fn main() {
    let x: Box<int> = box 10;
    let y = x;
    // attempting to use `x` will result in an error here
}