fn main() {
    use std::io;
    use std::num;
    
    // We derive `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `CliError` values.
    #[derive(Debug)]
    enum CliError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }
}
