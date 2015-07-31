fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    
    let some_isize = AtomicIsize::new(5);
    
    assert_eq!(some_isize.load(Ordering::Relaxed), 5);
}
