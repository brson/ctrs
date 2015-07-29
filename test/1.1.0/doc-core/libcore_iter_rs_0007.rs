fn main() {
    let a = [1, 2];
    let mut it = a.iter().map(|&x| 2 * x);
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(4));
    assert!(it.next().is_none());
}
