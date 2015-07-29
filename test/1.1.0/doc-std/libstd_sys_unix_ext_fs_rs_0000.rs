fn main() {
    use std::os::unix::fs;        fn foo() -> std::io::Result<()> {    try!(fs::symlink("a.txt", "b.txt"));    Ok(())    }}
