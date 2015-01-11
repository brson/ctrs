fn main() {
    let a = [100i, 200];
    let mut it = a.iter().enumerate();
    let (x100, x200) = (100i, 200i);
    assert_eq!(it.next().unwrap(), (0, &x100));
    assert_eq!(it.next().unwrap(), (1, &x200));
    assert!(it.next().is_none());
}
