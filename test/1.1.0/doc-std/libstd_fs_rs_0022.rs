fn main() {
    use std::fs;    fn foo() -> std::io::Result<()> {    for entry in try!(fs::read_dir(".")) {        let dir = try!(entry);        println!("{:?}", dir.path());    }    Ok(())    }}
