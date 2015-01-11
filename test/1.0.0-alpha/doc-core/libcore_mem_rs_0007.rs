fn main() {
    use std::mem;
    
    let x: int = unsafe { mem::uninitialized() };
}
