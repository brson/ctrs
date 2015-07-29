fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo.rs");        for component in path.components() {        println!("{:?}", component);    }}
