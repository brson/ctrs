fn main() {
    use std::collections::RingBuf;
    
    let mut buf: RingBuf<int> = vec![1].into_iter().collect();
    buf.reserve_exact(10);
    assert!(buf.capacity() >= 11);
}
