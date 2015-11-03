fn main() {
    enum CliError {
        Io(::std::io::Error),
        ParseInt(num::ParseIntError),
        ParseFloat(num::ParseFloatError),
    }
    
    use std::num;
    
    impl From<num::ParseFloatError> for CliError {
        fn from(err: num::ParseFloatError) -> CliError {
            CliError::ParseFloat(err)
        }
    }
}
