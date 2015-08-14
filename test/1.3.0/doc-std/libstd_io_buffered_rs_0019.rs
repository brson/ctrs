fn main() {
    use std::fs::File;
    use std::io::LineWriter;
    
    fn foo() -> std::io::Result<()> {
    let file = try!(File::create("poem.txt"));
    let file = LineWriter::new(file);
    
    let reference = file.get_ref();
    Ok(())
    }
}
