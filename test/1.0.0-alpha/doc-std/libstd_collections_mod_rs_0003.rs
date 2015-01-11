fn main() {
    use std::collections::RingBuf;
    
    let vec = vec![1u, 2, 3, 4];
    let buf: RingBuf<uint> = vec.into_iter().collect();
}
