fn main() {
    use std::num::CheckedMul;
    assert_eq!(5u8.checked_mul(&51), Some(255));
    assert_eq!(5u8.checked_mul(&52), None);
}
