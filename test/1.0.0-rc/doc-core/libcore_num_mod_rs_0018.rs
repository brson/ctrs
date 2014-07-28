fn main() {
    use std::num::CheckedDiv;
    assert_eq!((-127i8).checked_div(&-1), Some(127));
    assert_eq!((-128i8).checked_div(&-1), None);
}
