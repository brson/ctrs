fn main() {
    let a = [1i, 2, 3, 4, 5];
    let mut it = a.iter().take(3);
    assert_eq!(it.next().unwrap(), &1);
    assert_eq!(it.next().unwrap(), &2);
    assert_eq!(it.next().unwrap(), &3);
    assert!(it.next().is_none());
}
