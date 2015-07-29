fn main() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    let ptr = &mut 5;
    let some_ptr  = AtomicPtr::new(ptr);
    
    let other_ptr = &mut 10;
    
    let value = some_ptr.swap(other_ptr, Ordering::Relaxed);
}
