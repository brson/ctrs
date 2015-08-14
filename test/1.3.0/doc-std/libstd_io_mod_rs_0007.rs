fn main() {
    use std::io;
    
    fn foo() -> io::Result<()> {
    try!(io::copy(&mut io::stdin(), &mut io::stdout()));
    Ok(())
    }
}
