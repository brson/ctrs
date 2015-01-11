fn main() {
    use std::ptr;
    
    let p: *const int = ptr::null();
    assert!(p.is_null());
}
