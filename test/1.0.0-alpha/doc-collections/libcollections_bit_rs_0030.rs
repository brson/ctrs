fn main() {
    use std::collections::BitvSet;
    
    let mut s = BitvSet::with_capacity(100);
    assert!(s.capacity() >= 100);
}
