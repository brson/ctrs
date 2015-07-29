fn main() {
    use std::path::Path;
    
    assert_eq!(false, Path::new("foo.txt").is_absolute());
}
