fn main() {
    use std::collections::VecDeque;
    
    let mut buf: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
    let mut buf2: VecDeque<_> = vec![4, 5, 6].into_iter().collect();
    buf.append(&mut buf2);
    assert_eq!(buf.len(), 6);
    assert_eq!(buf2.len(), 0);
}
