fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    try!(fs::create_dir("/some/dir"));    Ok(())    }}
