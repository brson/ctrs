fn main() {
    use std::path::{Path, PathBuf};
    
    assert_eq!(Path::new("/etc").join("passwd"), PathBuf::from("/etc/passwd"));
}
