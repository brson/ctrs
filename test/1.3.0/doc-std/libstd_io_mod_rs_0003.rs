fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let f = try!(File::create("foo.txt"));
    {
        let mut writer = BufWriter::new(f);
    
        // write a byte to the buffer
        try!(writer.write(&[42]));
    
    } // the buffer is flushed once writer goes out of scope
    
    Ok(())
    }
}
