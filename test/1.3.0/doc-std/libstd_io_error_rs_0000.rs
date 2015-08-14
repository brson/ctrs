fn main() {
    use std::io;
    
    fn get_string() -> io::Result<String> {
        let mut buffer = String::new();
    
        try!(io::stdin().read_line(&mut buffer));
    
        Ok(buffer)
    }
}
