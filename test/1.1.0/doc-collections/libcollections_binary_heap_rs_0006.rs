fn main() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::with_capacity(100);
    assert!(heap.capacity() >= 100);
    heap.push(4);
}
