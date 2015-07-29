fn main() {
    let a = [100, 200];
    let mut it = a.iter().enumerate();
    assert_eq!(it.next(), Some((0, &100)));
    assert_eq!(it.next(), Some((1, &200)));
    assert!(it.next().is_none());
}
