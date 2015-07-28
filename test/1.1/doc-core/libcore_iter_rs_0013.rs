fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut it = a.iter().take_while(|&a| *a < 3);
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert!(it.next().is_none());
}
