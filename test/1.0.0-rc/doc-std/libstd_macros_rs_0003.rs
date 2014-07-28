fn main() {
    // the failure message for these assertions is the stringified value of the
    // expression given.
    debug_assert!(true);
    fn some_expensive_computation() -> bool { true }
    debug_assert!(some_expensive_computation());
    
    // assert with a custom message
    let x = true;
    debug_assert!(x, "x wasn't true!");
    let a = 3i; let b = 27i;
    debug_assert!(a + b == 30, "a = {}, b = {}", a, b);
}
