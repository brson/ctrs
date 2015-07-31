fn main() {
    let hello_vec = vec![104, 101, 108, 108, 111];
    let s = String::from_utf8(hello_vec).unwrap();
    assert_eq!(s, "hello");
    
    let invalid_vec = vec![240, 144, 128];
    let s = String::from_utf8(invalid_vec).err().unwrap();
    let err = s.utf8_error();
    assert_eq!(s.into_bytes(), [240, 144, 128]);
}
