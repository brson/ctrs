fn main() {
    use std::cmp::Ordering;        let result = 1.cmp(&2);    assert_eq!(Ordering::Less, result);        let result = 1.cmp(&1);    assert_eq!(Ordering::Equal, result);        let result = 2.cmp(&1);    assert_eq!(Ordering::Greater, result);}
