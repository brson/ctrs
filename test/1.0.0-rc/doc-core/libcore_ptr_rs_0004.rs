fn main() {
    use std::ptr;
    
    let p: *mut int = ptr::mut_null();
    assert!(p.is_null());
}
