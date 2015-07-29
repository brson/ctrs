fn main() {
    use std::path::PathBuf;        let mut path = PathBuf::from("c:\\");    path.push("windows");    path.push("system32");    path.set_extension("dll");}
