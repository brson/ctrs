fn main() {
    use std::path::Path;        let path = Path::new("/etc/passwd");        assert!(path.starts_with("/etc"));        assert!(!path.starts_with("/e"));}
