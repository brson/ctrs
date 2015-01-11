fn main() {
    let a = [1i, 2, 3, 2, 1];
    let mut it = a.iter().skip_while(|&a| *a < 3);
    assert_eq!(it.next().unwrap(), &3);
    assert_eq!(it.next().unwrap(), &2);
    assert_eq!(it.next().unwrap(), &1);
    assert!(it.next().is_none());
}
