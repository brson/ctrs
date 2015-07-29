fn main() {
    use std::fs;        fn foo() -> std::io::Result<()> {    let path = try!(fs::read_link("a.txt"));    Ok(())    }}
