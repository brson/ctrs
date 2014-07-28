fn main() {
    let x = 5i; // immutable
    let mut y = 5i; // mutable
    y += 2;
    
    let x = box 5i; // immutable
    let mut y = box 5i; // mutable
    *y += 2; // the `*` operator is needed to access the contained value
}