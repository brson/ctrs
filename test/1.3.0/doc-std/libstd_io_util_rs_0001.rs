fn main() {
    use std::io;
    use std::io::Read;
    
    fn foo() -> io::Result<String> {
    let mut buffer = String::new();
    try!(io::empty().read_to_string(&mut buffer));
    Ok(buffer)
    }
}
