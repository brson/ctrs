fn main() {
    use std::path::Path;
    
    let path_str = Path::new("foo.txt").to_string_lossy();
    assert_eq!(path_str, "foo.txt");
}
