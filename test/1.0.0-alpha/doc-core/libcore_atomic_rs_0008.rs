fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    let foo = AtomicBool::new(true);
    assert_eq!(true, foo.fetch_nand(false, Ordering::SeqCst));
    assert_eq!(true, foo.load(Ordering::SeqCst));
    
    let foo = AtomicBool::new(true);
    assert_eq!(true, foo.fetch_nand(true, Ordering::SeqCst));
    assert_eq!(0, foo.load(Ordering::SeqCst) as int);
    assert_eq!(false, foo.load(Ordering::SeqCst));
    
    let foo = AtomicBool::new(false);
    assert_eq!(false, foo.fetch_nand(false, Ordering::SeqCst));
    assert_eq!(true, foo.load(Ordering::SeqCst));
}
