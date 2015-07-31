fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    
    let foo = AtomicIsize::new(0);
    assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 0);
    assert_eq!(foo.load(Ordering::SeqCst), -10);
}
