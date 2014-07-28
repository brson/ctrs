fn main() {
    // The function is only included in the build when compiling for OSX
    #[cfg(target_os = "macos")]
    fn macos_only() {
      // ...
    }
    
    // This function is only included when either foo or bar is defined
    #[cfg(foo)]
    #[cfg(bar)]
    fn needs_foo_or_bar() {
      // ...
    }
    
    // This function is only included when compiling for a unixish OS with a 32-bit
    // architecture
    #[cfg(unix, target_word_size = "32")]
    fn on_32bit_unix() {
      // ...
    }
}