fn main() {
    // libstd/io/mod.rs
    
    pub use self::mem::{MemReader, BufReader, MemWriter, BufWriter};
    /* ... */
    mod mem;
}
