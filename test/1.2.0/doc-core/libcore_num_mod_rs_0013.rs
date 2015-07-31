fn main() {
    assert_eq!((-127i8).checked_sub(1), Some(-128));
    assert_eq!((-128i8).checked_sub(1), None);
}
