fn main() {
    // General metadata applied to the enclosing module or crate.
    #![license = "BSD"]
    
    // A function marked as a unit test
    #[test]
    fn test_foo() {
      /* ... */
    }
    
    // A conditionally-compiled module
    #[cfg(target_os="linux")]
    mod bar {
      /* ... */
    }
    
    // A lint attribute used to suppress a warning/error
    #[allow(non_camel_case_types)]
    type int8_t = i8;
}