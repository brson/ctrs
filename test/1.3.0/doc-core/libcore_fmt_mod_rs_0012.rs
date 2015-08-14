fn main() {
    let x = 42; // 42 is '2A' in hex
    
    assert_eq!(format!("{:X}", x), "2A");
    assert_eq!(format!("{:#X}", x), "0x2A");
}
