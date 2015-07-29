fn main() {
    let a = [1, 2, 3, 4, 5];    let mut it = a.iter();    assert_eq!(it.nth(2), Some(&3));    assert_eq!(it.nth(2), None);}
