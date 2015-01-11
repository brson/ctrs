fn main() {
    use std::collections::HashSet;
    let set: HashSet<int> = HashSet::with_capacity(100);
    assert!(set.capacity() >= 100);
}
