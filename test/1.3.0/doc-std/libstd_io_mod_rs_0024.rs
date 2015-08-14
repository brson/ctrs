fn main() {
    use std::io::Write;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut buffer = try!(File::create("foo.txt"));
    
    let reference = buffer.by_ref();
    
    // we can use reference just like our original buffer
    try!(reference.write_all(b"some bytes"));
    Ok(())
    }
}
