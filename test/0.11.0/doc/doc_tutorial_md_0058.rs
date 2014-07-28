fn main() {
    let x = box 5i;
    let y = x.clone(); // `y` is a newly allocated box
    let z = x; // no new memory allocated, `x` can no longer be used
}