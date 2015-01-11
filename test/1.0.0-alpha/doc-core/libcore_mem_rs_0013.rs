fn main() {
    use std::mem;
    
    let one = unsafe { mem::transmute_copy(&1i) };
    
    assert_eq!(1u, one);
}
