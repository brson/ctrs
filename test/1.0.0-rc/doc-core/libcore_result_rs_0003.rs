fn main() {
    use std::io::IoError;
    
    trait Writer {
        fn write_line(&mut self, s: &str) -> Result<(), IoError>;
    }
}
