fn main() {
    extern fn new_int() -> int { 0 }
    let fptr: extern "C" fn() -> int = new_int;
}