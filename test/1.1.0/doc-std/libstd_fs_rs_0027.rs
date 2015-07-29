fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    try!(fs::rename("a.txt", "b.txt"));    Ok(())    }}
