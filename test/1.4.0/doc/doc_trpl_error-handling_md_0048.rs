fn main() {
    use std::io;
    use std::num;
    
    enum CliError {
        Io(io::Error),
        ParseInt(num::ParseIntError),
        ParseFloat(num::ParseFloatError),
    }
}
