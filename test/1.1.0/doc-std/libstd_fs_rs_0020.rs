fn main() {
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut f = try!(File::create("foo.txt"));
    let metadata = try!(f.metadata());
    
    assert_eq!(false, metadata.permissions().readonly());
    Ok(())
    }
}
