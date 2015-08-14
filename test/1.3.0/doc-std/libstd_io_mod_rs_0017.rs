fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut buffer = [0; 5];
    
    // read at most five bytes
    let mut handle = f.take(5);
    
    try!(handle.read(&mut buffer));
    Ok(())
    }
}
