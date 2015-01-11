fn main() {
    let a = [1i, 2];
    let mut it = a.iter().map(|&x| 2 * x);
    assert_eq!(it.next().unwrap(), 2);
    assert_eq!(it.next().unwrap(), 4);
    assert!(it.next().is_none());
}
