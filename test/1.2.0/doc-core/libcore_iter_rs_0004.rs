fn main() {
    let a = [0];
    let b = [1];
    let mut it = a.iter().chain(&b);
    assert_eq!(it.next(), Some(&0));
    assert_eq!(it.next(), Some(&1));
    assert!(it.next().is_none());
}
