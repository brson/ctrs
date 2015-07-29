fn main() {
    let a = [1, 2];    let mut it = a.iter().filter(|&x| *x > 1);    assert_eq!(it.next(), Some(&2));    assert!(it.next().is_none());}
