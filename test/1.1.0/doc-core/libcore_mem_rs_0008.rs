fn main() {
    use std::mem;
    
    assert_eq!(4, mem::align_of_val(&5i32));
}
