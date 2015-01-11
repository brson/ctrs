fn main() {
    let a = [0i];
    let b = [1i];
    let mut it = a.iter().chain(b.iter());
    assert_eq!(it.next().unwrap(), &0);
    assert_eq!(it.next().unwrap(), &1);
    assert!(it.next().is_none());
}
