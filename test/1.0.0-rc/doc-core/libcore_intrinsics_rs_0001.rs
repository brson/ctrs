fn main() {
    use std::mem;
    use std::ptr;
    
    fn swap<T>(x: &mut T, y: &mut T) {
        unsafe {
            // Give ourselves some scratch space to work with
            let mut t: T = mem::uninitialized();
    
            // Perform the swap, `&mut` pointers never alias
            ptr::copy_nonoverlapping_memory(&mut t, &*x, 1);
            ptr::copy_nonoverlapping_memory(x, &*y, 1);
            ptr::copy_nonoverlapping_memory(y, &t, 1);
    
            // y and t now point to the same thing, but we need to completely forget `tmp`
            // because it's no longer relevant.
            mem::forget(t);
        }
    }
}
