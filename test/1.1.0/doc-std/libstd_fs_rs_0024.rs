fn main() {
    use std::fs;
    
    fn foo() -> std::io::Result<()> {
    try!(fs::remove_file("a.txt"));
    Ok(())
    }
}
