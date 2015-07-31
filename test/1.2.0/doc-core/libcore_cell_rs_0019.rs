fn main() {
    use std::cell::UnsafeCell;
    
    let uc = UnsafeCell::new(5);
    
    let five = unsafe { uc.into_inner() };
}
