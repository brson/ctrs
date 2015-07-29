fn main() {
    let a = [1, 2, 3, 4, 5];    let mut it = a.iter().take(3);    assert_eq!(it.next(), Some(&1));    assert_eq!(it.next(), Some(&2));    assert_eq!(it.next(), Some(&3));    assert!(it.next().is_none());}
