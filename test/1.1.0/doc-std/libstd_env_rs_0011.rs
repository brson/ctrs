fn main() {
    use std::env;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut dir = env::temp_dir();
    dir.push("foo.txt");
    
    let f = try!(File::create(dir));
    Ok(())
    }
}
