fn main() {
    use std::io::{self, Read};
    
    fn foo() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    try!(handle.read_to_string(&mut buffer));
    Ok(buffer)
    }
}
