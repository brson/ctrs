fn main() {
    use std::collections::VecDeque;
    
    let buf: VecDeque<i32> = VecDeque::with_capacity(10);
    assert!(buf.capacity() >= 10);
}
