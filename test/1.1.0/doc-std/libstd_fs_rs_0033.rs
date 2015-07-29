fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    try!(fs::create_dir_all("/some/dir"));    Ok(())    }}
