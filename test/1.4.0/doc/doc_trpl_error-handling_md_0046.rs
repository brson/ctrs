fn main() {
    #[derive(Debug)]
    enum CliError { Io(io::Error), Parse(num::ParseIntError) }
    use std::io;
    use std::num;
    
    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::Io(err)
        }
    }
    
    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::Parse(err)
        }
    }
}
