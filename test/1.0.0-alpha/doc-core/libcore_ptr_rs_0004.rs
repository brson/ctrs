fn main() {
    use std::ptr;
    
    let p: *mut int = ptr::null_mut();
    assert!(p.is_null());
}
