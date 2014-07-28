fn main() {
    let a = [1i, 2, 3, 4, 5];
    assert!(a.iter().fold(0, |a, &b| a + b) == 15);
}
