fn main() {
    use std::mem;
    
    let heap_memory = Box::new(3);
    mem::forget(heap_memory);
}
