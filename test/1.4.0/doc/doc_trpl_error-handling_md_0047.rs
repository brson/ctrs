fn main() {
    use std::io;
    use std::num;
    enum CliError { Io(::std::io::Error), Parse(::std::num::ParseIntError) }
    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError { CliError::Io(err) }
    }
    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError { CliError::Parse(err) }
    }
    
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    
    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut file = try!(File::open(file_path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        let n: i32 = try!(contents.trim().parse());
        Ok(2 * n)
    }
}
