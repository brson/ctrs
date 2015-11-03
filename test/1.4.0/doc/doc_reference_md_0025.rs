fn main() {
    // Declares an extern fn, the ABI defaults to "C"
    extern fn new_i32() -> i32 { 0 }
    
    // Declares an extern fn with "stdcall" ABI
    extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }
}
