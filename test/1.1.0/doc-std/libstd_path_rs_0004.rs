fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo/bar.txt");        println!("{:?}", path.components().as_path());}
