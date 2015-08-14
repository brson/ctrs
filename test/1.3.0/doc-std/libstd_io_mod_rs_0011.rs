fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut buffer = Vec::new();
    
    // read the whole file
    try!(f.read_to_end(&mut buffer));
    Ok(())
    }
}
