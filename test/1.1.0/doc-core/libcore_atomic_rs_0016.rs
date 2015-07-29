fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    
    let foo = AtomicIsize::new(0);
    assert_eq!(0, foo.fetch_add(10, Ordering::SeqCst));
    assert_eq!(10, foo.load(Ordering::SeqCst));
}
