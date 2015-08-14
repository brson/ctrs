fn main() {
    use std::mem;
    use std::ptr;
    
    // Only declare the array. This safely leaves it
    // uninitialized in a way that Rust will track for us.
    // However we can't initialize it element-by-element
    // safely, and we can't use the `[value; 1000]`
    // constructor because it only works with `Copy` data.
    let mut data: [Vec<u32>; 1000];
    
    unsafe {
        // So we need to do this to initialize it.
        data = mem::uninitialized();
    
        // DANGER ZONE: if anything panics or otherwise
        // incorrectly reads the array here, we will have
        // Undefined Behaviour.
    
        // It's ok to mutably iterate the data, since this
        // doesn't involve reading it at all.
        // (ptr and len are statically known for arrays)
        for elem in &mut data[..] {
            // *elem = Vec::new() would try to drop the
            // uninitialized memory at `elem` -- bad!
            //
            // Vec::new doesn't allocate or do really
            // anything. It's only safe to call here
            // because we know it won't panic.
            ptr::write(elem, Vec::new());
        }
    
        // SAFE ZONE: everything is initialized.
    }
    
    println!("{:?}", &data[0]);
}
