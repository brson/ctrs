fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    
    let some_isize = AtomicIsize::new(5);
    
    assert_eq!(some_isize.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    assert_eq!(some_isize.load(Ordering::Relaxed), 10);
    
    assert_eq!(some_isize.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    assert_eq!(some_isize.load(Ordering::Relaxed), 10);
}
