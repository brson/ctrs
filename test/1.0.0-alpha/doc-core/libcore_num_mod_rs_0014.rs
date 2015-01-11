fn main() {
    use std::num::Int;
    
    assert_eq!((-127i8).checked_div(-1), Some(127));
    assert_eq!((-128i8).checked_div(-1), None);
    assert_eq!((1i8).checked_div(0), None);
}
