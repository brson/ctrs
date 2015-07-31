fn main() {
    use std::collections::VecDeque;
    
    let mut buf: VecDeque<i32> = vec![1].into_iter().collect();
    buf.reserve(10);
    assert!(buf.capacity() >= 11);
}
