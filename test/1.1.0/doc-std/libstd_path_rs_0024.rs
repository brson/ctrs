fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo.rs");        let new_path = path.with_file_name("bar.rs");}
