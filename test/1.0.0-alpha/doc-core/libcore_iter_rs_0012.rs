fn main() {
    let a = [1i, 2, 3, 2, 1];
    let mut it = a.iter().take_while(|&a| *a < 3);
    assert_eq!(it.next().unwrap(), &1);
    assert_eq!(it.next().unwrap(), &2);
    assert!(it.next().is_none());
}
