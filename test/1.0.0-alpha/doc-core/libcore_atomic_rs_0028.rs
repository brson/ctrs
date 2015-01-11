fn main() {
    use std::sync::atomic::AtomicPtr;
    
    let ptr = &mut 5i;
    let atomic_ptr  = AtomicPtr::new(ptr);
}
