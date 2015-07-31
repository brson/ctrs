fn main() {
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let f = try!(File::create("foo.txt"));
    let metadata = try!(f.metadata());
    let mut permissions = metadata.permissions();
    
    permissions.set_readonly(true);
    
    // filesystem doesn't change
    assert_eq!(false, metadata.permissions().readonly());
    
    // just this particular `permissions`.
    assert_eq!(true, permissions.readonly());
    Ok(())
    }
}
