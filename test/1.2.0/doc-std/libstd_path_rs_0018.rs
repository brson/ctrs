fn main() {
    use std::path::Path;
    use std::ffi::OsStr;
    
    let path = Path::new("foo.txt");
    let os_str = OsStr::new("foo.txt");
    
    assert_eq!(Some(os_str), path.file_name());
}
