fn main() {
    use std::mem;
    
    let one = unsafe { mem::transmute_copy(&1) };
    
    assert_eq!(1, one);
}
