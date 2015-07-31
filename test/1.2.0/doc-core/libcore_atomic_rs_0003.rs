fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    let some_bool = AtomicBool::new(true);
    
    assert_eq!(some_bool.load(Ordering::Relaxed), true);
}
