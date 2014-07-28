fn main() {
    #![allow(unused_attribute)]
    // `lib.rs`
    // ...
    
    // This crate is a library ("bin" is the default)
    #![crate_id = "farm#2.5"]
    #![crate_type = "lib"]
    
    // Turn on a warning
    #[warn(non_camel_case_types)]
    fn farm() {}
}