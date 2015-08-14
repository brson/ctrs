fn main() {
    use std::fs::File;
    use std::io::LineWriter;
    
    fn foo() -> std::io::Result<()> {
    let file = try!(File::create("poem.txt"));
    
    let writer: LineWriter<File> = LineWriter::new(file);
    
    let file: File = try!(writer.into_inner());
    Ok(())
    }
}
