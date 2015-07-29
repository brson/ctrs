fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    try!(fs::hard_link("a.txt", "b.txt"));    Ok(())    }}
