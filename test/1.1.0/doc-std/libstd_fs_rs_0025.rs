fn main() {
    fn foo() -> std::io::Result<()> {
    use std::fs;
    
    let attr = try!(fs::metadata("/some/file/path.txt"));
    // inspect attr ...
    Ok(())
    }
}
