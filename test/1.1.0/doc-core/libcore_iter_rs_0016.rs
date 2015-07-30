fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut it = a.iter().scan(1, |fac, &x| {
    *fac = *fac * x;
    Some(*fac)
    });
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(6));
    assert_eq!(it.next(), Some(24));
    assert_eq!(it.next(), Some(120));
    assert!(it.next().is_none());
}
