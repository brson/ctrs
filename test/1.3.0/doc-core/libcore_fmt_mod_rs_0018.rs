fn main() {
    let x = 42.0; // 42.0 is '4.2E1' in scientific notation
    
    assert_eq!(format!("{:E}", x), "4.2E1");
}
