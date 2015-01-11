fn main() {
    let x: &mut[int] = &mut [1i, 2, 3];
    x[1] = 7;
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 7);
    assert_eq!(x[2], 3);
}
