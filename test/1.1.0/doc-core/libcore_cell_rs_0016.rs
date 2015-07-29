fn main() {
    use std::cell::UnsafeCell;
    
    let uc = UnsafeCell::new(5);
    
    let five = uc.get();
}
