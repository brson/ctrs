fn main() {
    use std::mem;
    use std::ptr;
    
    // size of the array is hard-coded but easy to change. This means we can't
    // use [a, b, c] syntax to initialize the array, though!
    const SIZE: usize = 10;
    
    let mut x: [Box<u32>; SIZE];
    
    unsafe {
    	// convince Rust that x is Totally Initialized
    	x = mem::uninitialized();
    	for i in 0..SIZE {
    		// very carefully overwrite each index without reading it
    		// NOTE: exception safety is not a concern; Box can't panic
    		ptr::write(&mut x[i], Box::new(i as u32));
    	}
    }
    
    println!("{:?}", x);
}
