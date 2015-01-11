fn main() {
    let a = [0i];
    let b = [1i];
    let mut it = a.iter().zip(b.iter());
    let (x0, x1) = (0i, 1i);
    assert_eq!(it.next().unwrap(), (&x0, &x1));
    assert!(it.next().is_none());
}
