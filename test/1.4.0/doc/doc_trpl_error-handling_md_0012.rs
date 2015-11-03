fn main() {
    fn extension(file_name: &str) -> Option<&str> { None }
    fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
        match file_name(file_path) {
            None => None,
            Some(name) => match extension(name) {
                None => None,
                Some(ext) => Some(ext),
            }
        }
    }
    
    fn file_name(file_path: &str) -> Option<&str> {
      // implementation elided
      unimplemented!()
    }
}
