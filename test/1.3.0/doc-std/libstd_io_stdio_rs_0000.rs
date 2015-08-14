fn main() {
    use std::io::{self, Read};
    
    fn foo() -> io::Result<String> {
    let mut buffer = String::new();
    try!(io::stdin().read_to_string(&mut buffer));
    Ok(buffer)
    }
}
