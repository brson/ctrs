fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    let foo = AtomicBool::new(true);
    assert_eq!(true, foo.fetch_or(false, Ordering::SeqCst));
    assert_eq!(true, foo.load(Ordering::SeqCst));
    
    let foo = AtomicBool::new(true);
    assert_eq!(true, foo.fetch_or(true, Ordering::SeqCst));
    assert_eq!(true, foo.load(Ordering::SeqCst));
    
    let foo = AtomicBool::new(false);
    assert_eq!(false, foo.fetch_or(false, Ordering::SeqCst));
    assert_eq!(false, foo.load(Ordering::SeqCst));
}
