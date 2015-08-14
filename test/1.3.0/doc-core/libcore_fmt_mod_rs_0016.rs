fn main() {
    let x = 42.0; // 42.0 is '4.2e1' in scientific notation
    
    assert_eq!(format!("{:e}", x), "4.2e1");
}
