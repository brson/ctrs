fn main() {
    struct MyBox(*mut u8);
    
    unsafe impl Send for MyBox {}
    unsafe impl Sync for MyBox {}
}
