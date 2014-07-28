fn main() {
    // Declares an extern fn, the ABI defaults to "C"
    extern fn new_int() -> int { 0 }
    
    // Declares an extern fn with "stdcall" ABI
    extern "stdcall" fn new_int_stdcall() -> int { 0 }
}