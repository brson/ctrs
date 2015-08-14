fn main() {
    let x = 42; // 42 is '52' in octal
    
    assert_eq!(format!("{:o}", x), "52");
    assert_eq!(format!("{:#o}", x), "0o52");
}
