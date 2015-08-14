fn main() {
    let x = 42; // 42 is '2a' in hex
    
    assert_eq!(format!("{:x}", x), "2a");
    assert_eq!(format!("{:#x}", x), "0x2a");
}
