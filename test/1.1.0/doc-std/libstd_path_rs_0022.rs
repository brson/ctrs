fn main() {
    use std::path::Path;
    
    let path = Path::new("foo.rs");
    
    assert_eq!("rs", path.extension().unwrap());
}
