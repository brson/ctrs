fn main() {
    use std::cmp::Ordering::{Less, Equal, Greater};
    
    assert_eq!( 5u.cmp(&10), Less);     // because 5 < 10
    assert_eq!(10u.cmp(&5),  Greater);  // because 10 > 5
    assert_eq!( 5u.cmp(&5),  Equal);    // because 5 == 5
}
