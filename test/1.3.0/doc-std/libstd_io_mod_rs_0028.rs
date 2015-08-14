fn main() {
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let f = try!(File::open("foo.txt"));
    let f = BufReader::new(f);
    
    for line in f.lines() {
        println!("{}", line.unwrap());
    }
    
    Ok(())
    }
}
