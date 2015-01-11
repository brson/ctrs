fn main() {
    let a = [1i, 2, 3, 4, 5];
    let mut it = a.iter();
    assert!(it.any(|x| *x == 3));
    assert!(!it.any(|x| *x == 3));
}
