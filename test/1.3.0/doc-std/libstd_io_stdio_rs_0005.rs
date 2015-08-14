fn main() {
    use std::io::{self, Write};
    
    fn foo() -> io::Result<()> {
    try!(io::stderr().write(b"hello world"));
    
    Ok(())
    }
}
