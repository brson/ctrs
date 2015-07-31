fn main() {
    use std::slice;
    
    // manifest a slice out of thin air!
    let ptr = 0x1234 as *const usize;
    let amt = 10;
    unsafe {
    let slice = slice::from_raw_parts(ptr, amt);
    }
}
