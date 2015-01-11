fn main() {
    let a = [1i, 2, 3, 4, 5];
    let mut it = a.iter().skip(3);
    assert_eq!(it.next().unwrap(), &4);
    assert_eq!(it.next().unwrap(), &5);
    assert!(it.next().is_none());
}
