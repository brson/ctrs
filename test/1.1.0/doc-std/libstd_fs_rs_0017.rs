fn main() {
    fn foo() -> std::io::Result<()> {    use std::fs;        let metadata = try!(fs::metadata("foo.txt"));        assert!(metadata.is_file());    Ok(())    }}
