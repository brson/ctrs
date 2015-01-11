fn main() {
    let mut vec = vec!(1i, 2);
    vec.push(3);
    assert_eq!(vec, vec!(1, 2, 3));
}
