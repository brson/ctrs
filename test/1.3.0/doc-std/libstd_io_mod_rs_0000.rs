fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut buffer = [0; 10];
    
    // read up to 10 bytes
    try!(f.read(&mut buffer));
    
    println!("The bytes: {:?}", buffer);
    Ok(())
    }
}
