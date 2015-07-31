fn main() {
    use std::path::Path;
    
    let path = Path::new("/etc/passwd");
    
    assert!(path.ends_with("passwd"));
}
