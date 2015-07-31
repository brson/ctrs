fn main() {
    use std::path::Path;
    
    assert!(Path::new("/etc/passwd").has_root());
}
