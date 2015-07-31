fn main() {
    use std::mem;
    
    let v: &[u8] = unsafe { mem::transmute("L") };
    assert!(v == [76]);
}
