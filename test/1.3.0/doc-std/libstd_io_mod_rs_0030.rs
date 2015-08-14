fn main() {
    use std::io;
    use std::io::prelude::*;
    
    fn foo() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    
    try!(stdin.read_until(b'a', &mut buffer));
    
    println!("{:?}", buffer);
    Ok(())
    }
}
