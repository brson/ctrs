fn main() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    let ptr = &mut 5;
    let some_ptr  = AtomicPtr::new(ptr);
    
    let value = some_ptr.load(Ordering::Relaxed);
}
