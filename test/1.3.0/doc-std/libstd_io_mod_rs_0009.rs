fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut buffer = [0; 10];
    
    // read up to 10 bytes
    try!(f.read(&mut buffer));
    
    let mut buffer = vec![0; 10];
    // read the whole file
    try!(f.read_to_end(&mut buffer));
    
    // read into a String, so that you don't need to do the conversion.
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));
    
    // and more! See the other methods for more details.
    Ok(())
    }
}
