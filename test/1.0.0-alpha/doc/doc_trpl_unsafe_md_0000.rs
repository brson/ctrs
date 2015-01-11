fn main() {
    use std::mem;

    let mut x: u8 = 1;

    

    let ref_1: &mut u8 = &mut x;

    let ref_2: &mut u8 = unsafe { mem::transmute(&mut *ref_1) };

    

    // oops, ref_1 and ref_2 point to the same piece of data (x) and are

    // both usable

    *ref_1 = 10;

    *ref_2 = 20;

}
