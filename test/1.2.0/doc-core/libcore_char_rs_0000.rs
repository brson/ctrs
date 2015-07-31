fn main() {
    use std::char;
    
    assert_eq!(char::from_u32(0x2764), Some('❤'));
    assert_eq!(char::from_u32(0x110000), None); // invalid character
}
