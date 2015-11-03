fn main() {
    use std::mem;
    
    let array: &[u8] = unsafe { mem::transmute("Rust") };
    assert_eq!(array, [82, 117, 115, 116]);
}
