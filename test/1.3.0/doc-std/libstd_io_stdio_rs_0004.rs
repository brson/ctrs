fn main() {
    use std::io::{self, Write};
    
    fn foo() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    try!(handle.write(b"hello world"));
    
    Ok(())
    }
}
