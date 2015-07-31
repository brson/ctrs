fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut it = a.iter().skip_while(|&a| *a < 3);
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&4));
    assert_eq!(it.next(), Some(&5));
    assert!(it.next().is_none());
}
