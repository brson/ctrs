fn main() {
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(try!(File::create("foo.txt")));
    
    try!(buffer.write(b"some bytes"));
    try!(buffer.flush());
    Ok(())
    }
}
