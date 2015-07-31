fn main() {
    use std::path::Path;
    
    assert!(!Path::new("foo.txt").is_absolute());
}
