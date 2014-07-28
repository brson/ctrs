fn main() {
    fn call_twice(f: ||) { f(); f(); }
    let closure = || { "I'm a closure, and it doesn't matter what type I am"; };
    fn function() { "I'm a normal function"; }
    call_twice(closure);
    call_twice(function);
}