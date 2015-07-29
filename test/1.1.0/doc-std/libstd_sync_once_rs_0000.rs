fn main() {
    use std::sync::{Once, ONCE_INIT};
    
    static START: Once = ONCE_INIT;
    
    START.call_once(|| {
        // run initialization here
    });
}
