fn main() {
    use std::io::BufReader;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut f1 = try!(File::open("log.txt"));
    let mut reader = BufReader::new(f1);
    
    let f2 = reader.get_mut();
    Ok(())
    }
}
