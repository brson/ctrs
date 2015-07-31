fn main() {
    use std::path::{Path, PathBuf};
    
    let path = Path::new("/tmp/foo.txt");
    assert_eq!(path.with_file_name("bar.txt"), PathBuf::from("/tmp/bar.txt"));
}
