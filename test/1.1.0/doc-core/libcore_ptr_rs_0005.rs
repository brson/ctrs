fn main() {
    use std::ptr;
    
    let p: *mut i32 = ptr::null_mut();
    assert!(p.is_null());
}
