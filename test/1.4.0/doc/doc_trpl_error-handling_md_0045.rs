fn main() {
    use std::fs::File;
    use std::io::{self, Read};
    use std::num;
    use std::path::Path;
    
    // We derive `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `CliError` values.
    #[derive(Debug)]
    enum CliError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }
    
    fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut file = try!(File::open(file_path).map_err(CliError::Io));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents).map_err(CliError::Io));
        let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
        Ok(2 * n)
    }
}
