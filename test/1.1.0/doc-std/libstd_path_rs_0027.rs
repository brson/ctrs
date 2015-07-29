fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo.rs");        for component in path.iter() {        println!("{:?}", component);    }}
