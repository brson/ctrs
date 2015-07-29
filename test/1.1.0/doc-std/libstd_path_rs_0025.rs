fn main() {
    use std::path::{Path, PathBuf};
    
    let path = Path::new("/tmp/foo.rs");
    
    let new_path = path.with_extension("txt");
    assert_eq!(new_path, PathBuf::from("/tmp/foo.txt"));
}
