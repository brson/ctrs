fn main() {
    use std::path::{Path, PathBuf};
    
    let path = Path::new("foo.rs");
    assert_eq!(path.with_extension("txt"), PathBuf::from("foo.txt"));
}
