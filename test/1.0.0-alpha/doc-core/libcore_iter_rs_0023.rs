fn main() {
    let a = [1i, 2, 3, 4, 5];
    assert!(a.iter().all(|x| *x > 0));
    assert!(!a.iter().all(|x| *x > 2));
}
