fn main() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.reserve(100);
    assert!(heap.capacity() >= 100);
    heap.push(4u);
}
