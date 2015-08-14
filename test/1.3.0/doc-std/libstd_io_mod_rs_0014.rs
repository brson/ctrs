fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    
    for byte in f.bytes() {
        println!("{}", byte.unwrap());
    }
    Ok(())
    }
}
