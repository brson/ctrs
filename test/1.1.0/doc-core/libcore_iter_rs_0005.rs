fn main() {
    let a = [0];    let b = [1];    let mut it = a.iter().zip(b.iter());    assert_eq!(it.next(), Some((&0, &1)));    assert!(it.next().is_none());}
