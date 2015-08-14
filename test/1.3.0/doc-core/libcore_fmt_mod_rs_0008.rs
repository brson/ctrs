fn main() {
    let x = 42; // 42 is '101010' in binary
    
    assert_eq!(format!("{:b}", x), "101010");
    assert_eq!(format!("{:#b}", x), "0b101010");
}
