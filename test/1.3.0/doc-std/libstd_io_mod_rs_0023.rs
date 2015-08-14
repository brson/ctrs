fn main() {
    use std::io::prelude::*;
    use std::fs::File;
    
    fn foo() -> std::io::Result<()> {
    let mut buffer = try!(File::create("foo.txt"));
    
    // this call
    try!(write!(buffer, "{:.*}", 2, 1.234567));
    // turns into this:
    try!(buffer.write_fmt(format_args!("{:.*}", 2, 1.234567)));
    Ok(())
    }
}
