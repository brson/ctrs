fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let f = try!(File::open("foo.txt"));
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    
    // read a line into buffer
    try!(reader.read_line(&mut buffer));
    
    println!("{}", buffer);
    Ok(())
    }
}
