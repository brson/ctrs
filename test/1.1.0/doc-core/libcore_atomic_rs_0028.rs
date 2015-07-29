fn main() {
    use std::sync::atomic::AtomicPtr;        let ptr = &mut 5;    let atomic_ptr  = AtomicPtr::new(ptr);}
