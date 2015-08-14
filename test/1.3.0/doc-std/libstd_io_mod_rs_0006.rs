fn main() {
    use std::io;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    
    fn foo() -> io::Result<()> {
    let f = try!(File::open("foo.txt"));
    let mut reader = BufReader::new(f);
    
    for line in reader.lines() {
        let line = try!(line);
        println!("{}", line);
    }
    
    Ok(())
    }
}
