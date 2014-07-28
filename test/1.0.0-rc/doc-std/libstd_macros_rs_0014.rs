fn main() {
    let rust = bytes!("r", 'u', "st", 255);
    assert_eq!(rust[1], 'u' as u8);
    assert_eq!(rust[4], 255);
}
