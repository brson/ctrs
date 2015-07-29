fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    try!(fs::remove_dir_all("/some/dir"));    Ok(())    }}
