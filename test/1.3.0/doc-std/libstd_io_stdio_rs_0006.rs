fn main() {
    use std::io::{self, Write};
    
    fn foo() -> io::Result<()> {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    
    try!(handle.write(b"hello world"));
    
    Ok(())
    }
}
