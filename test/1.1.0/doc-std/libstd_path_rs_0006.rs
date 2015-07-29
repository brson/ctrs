fn main() {
    use std::path::PathBuf;
    
    let mut buf = PathBuf::from("/");
    assert!(buf.file_name() == None);
    buf.set_file_name("bar");
    assert!(buf == PathBuf::from("/bar"));
    assert!(buf.file_name().is_some());
    buf.set_file_name("baz.txt");
    assert!(buf == PathBuf::from("/baz.txt"));
}
