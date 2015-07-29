fn main() {
    use std::path::Path;
    
    let path = Path::new("foo.rs");
    
    assert_eq!("foo", path.file_stem().unwrap());
}
