fn main() {
    use std::mem;
    
    assert_eq!(4, mem::size_of_val(&5i32));
}
