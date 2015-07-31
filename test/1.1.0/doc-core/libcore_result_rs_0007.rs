fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    fn write_message() -> io::Result<()> {
        let mut file = try!(File::create("valuable_data.txt"));
        try!(file.write_all(b"important message"));
        Ok(())
    }
}
