fn main() {
    use std::io::BufReader;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut f = try!(File::open("log.txt"));
    let mut reader = BufReader::new(f);
    Ok(())
    }
}
