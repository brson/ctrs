fn main() {
    use std::fs::File;
    use std::io::LineWriter;
    
    fn foo() -> std::io::Result<()> {
    let file = try!(File::create("poem.txt"));
    let mut file = LineWriter::new(file);
    
    // we can use reference just like file
    let reference = file.get_mut();
    Ok(())
    }
}
