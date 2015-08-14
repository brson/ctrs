fn main() {
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut buffer = try!(File::create("foo.txt"));
    
    try!(buffer.write(b"some bytes"));
    Ok(())
    }
}
