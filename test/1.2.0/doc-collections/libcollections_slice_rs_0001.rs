fn main() {
    let x = &mut [1, 2, 3];
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);
}
