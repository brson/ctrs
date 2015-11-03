fn main() {
    use std::path::Path;
    
    let mut components = Path::new("/tmp/foo/bar.txt").components();
    components.next();
    components.next();
    
    assert_eq!(Path::new("foo/bar.txt"), components.as_path());
}
