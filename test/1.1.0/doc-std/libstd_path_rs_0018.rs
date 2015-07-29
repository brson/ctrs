fn main() {
    use std::path::Path;
    
    let path = Path::new("hello_world.rs");
    let filename = "hello_world.rs";
    
    assert_eq!(filename, path.file_name().unwrap());
}
