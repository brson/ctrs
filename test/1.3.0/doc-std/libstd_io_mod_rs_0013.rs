fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut buffer = Vec::new();
    let mut other_buffer = Vec::new();
    
    {
        let reference = f.by_ref();
    
        // read at most 5 bytes
        try!(reference.take(5).read_to_end(&mut buffer));
    
    } // drop our &mut reference so we can use f again
    
    // original file still usable, read the rest
    try!(f.read_to_end(&mut other_buffer));
    Ok(())
    }
}
