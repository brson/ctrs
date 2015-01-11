fn main() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    let ptr = &mut 5i;
    let some_ptr  = AtomicPtr::new(ptr);
    
    let other_ptr = &mut 10i;
    
    some_ptr.store(other_ptr, Ordering::Relaxed);
}
