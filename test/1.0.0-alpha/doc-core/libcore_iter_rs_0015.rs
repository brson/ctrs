fn main() {
    let a = [1i, 2, 3, 4, 5];
    let mut it = a.iter().scan(1, |fac, &x| {
      *fac = *fac * x;
      Some(*fac)
    });
    assert_eq!(it.next().unwrap(), 1);
    assert_eq!(it.next().unwrap(), 2);
    assert_eq!(it.next().unwrap(), 6);
    assert_eq!(it.next().unwrap(), 24);
    assert_eq!(it.next().unwrap(), 120);
    assert!(it.next().is_none());
}
