fn main() {
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
    let complete = &a[..]; // A slice containing all of the elements in a
}
