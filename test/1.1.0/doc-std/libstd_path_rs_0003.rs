fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo/bar.txt");        for component in path.components() {        println!("{:?}", component);    }}
