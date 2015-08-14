fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::SeekFrom;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    
    // move the cursor 42 bytes from the start of the file
    try!(f.seek(SeekFrom::Start(42)));
    Ok(())
    }
}
