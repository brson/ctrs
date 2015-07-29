fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut it = a.iter();
    assert!(it.any(|x| *x == 3));
    assert_eq!(it.collect::<Vec<_>>(), [&4, &5]);
}
