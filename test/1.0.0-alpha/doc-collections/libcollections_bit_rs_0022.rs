fn main() {
    use std::collections::Bitv;
    
    let mut bv = Bitv::new();
    bv.reserve(10);
    assert!(bv.capacity() >= 10);
}
