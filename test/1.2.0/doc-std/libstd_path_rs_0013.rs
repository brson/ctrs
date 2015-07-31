fn main() {
    use std::path::Path;
    
    let path_buf = Path::new("foo.txt").to_path_buf();
    assert_eq!(path_buf, std::path::PathBuf::from("foo.txt"));
}
