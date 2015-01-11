fn main() {
    use std::collections::RingBuf;
    
    let buf: RingBuf<int> = RingBuf::with_capacity(10);
    assert!(buf.capacity() >= 10);
}
