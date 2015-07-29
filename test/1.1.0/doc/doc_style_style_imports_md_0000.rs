fn main() {
    // Crates.
    extern crate getopts;
    extern crate mylib;
    
    // Standard library imports.
    use getopts::{optopt, getopts};
    use std::os;
    
    // Import from a library that we wrote.
    use mylib::webserver;
    
    // Will be reexported when we import this module.
    pub use self::types::Webdata;
}
