fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    
    let foo = AtomicIsize::new(0);
    assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    assert_eq!(foo.load(Ordering::SeqCst), 10);
}
