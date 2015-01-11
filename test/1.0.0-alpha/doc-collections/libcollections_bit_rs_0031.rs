fn main() {
    use std::collections::BitvSet;
    
    let mut s = BitvSet::new();
    s.reserve_len(10);
    assert!(s.capacity() >= 10);
}
