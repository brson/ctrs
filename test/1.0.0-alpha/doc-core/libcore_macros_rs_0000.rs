fn main() {
    // the panic message for these assertions is the stringified value of the
    // expression given.
    assert!(true);
    fn some_computation() -> bool { true }
    assert!(some_computation());
    
    // assert with a custom message
    let x = true;
    assert!(x, "x wasn't true!");
    let a = 3i; let b = 27i;
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}
