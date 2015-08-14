fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let mut f1 = try!(File::open("foo.txt"));
    let mut f2 = try!(File::open("bar.txt"));
    
    let mut handle = f1.chain(f2);
    let mut buffer = String::new();
    
    // read the value into a String. We could use any Read method here,
    // this is just one example.
    try!(handle.read_to_string(&mut buffer));
    Ok(())
    }
}
