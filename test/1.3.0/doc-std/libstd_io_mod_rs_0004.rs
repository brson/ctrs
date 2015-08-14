fn main() {
    use std::io;
    
    fn foo() -> io::Result<()> {
    let mut input = String::new();
    
    try!(io::stdin().read_line(&mut input));
    
    println!("You typed: {}", input.trim());
    Ok(())
    }
}
