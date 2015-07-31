fn main() {
    use std::mem;
    use std::ptr;
    
    fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
    // Give ourselves some scratch space to work with
    let mut t: T = mem::uninitialized();
    
    // Perform the swap, `&mut` pointers never alias
    ptr::copy_nonoverlapping(&*x, &mut t, 1);
    ptr::copy_nonoverlapping(&*y, x, 1);
    ptr::copy_nonoverlapping(&t, y, 1);
    
    // y and t now point to the same thing, but we need to completely
    // forget `t` because we do not want to run the destructor for `T`
    // on its value, which is still owned somewhere outside this function.
    mem::forget(t);
    }
    }
}
