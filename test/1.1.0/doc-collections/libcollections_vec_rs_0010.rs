use std::ptr;
use std::mem;

fn main() {
    let mut v = vec![1, 2, 3];

    // Pull out the various important pieces of information about `v`
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Cast `v` into the void: no destructor run, so we are in
        // complete control of the allocation to which `p` points.
        mem::forget(v);

        // Overwrite memory with 4, 5, 6
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}
