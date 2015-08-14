fn main() {
    use std::io::{self, Write};
    
    fn foo() -> io::Result<()> {
    try!(io::stdout().write(b"hello world"));
    
    Ok(())
    }
}
