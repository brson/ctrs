fn main() {
    use std::io;
    use std::io::prelude::*;
    
    fn foo() -> io::Result<()> {
    try!(io::stdout().write(&[42]));
    Ok(())
    }
}
