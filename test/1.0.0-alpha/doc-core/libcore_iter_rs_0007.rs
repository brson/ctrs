fn main() {
    let a = [1i, 2];
    let mut it = a.iter().filter(|&x| *x > 1);
    assert_eq!(it.next().unwrap(), &2);
    assert!(it.next().is_none());
}
