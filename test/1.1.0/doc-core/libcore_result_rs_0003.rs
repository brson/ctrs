fn main() {
    use std::io;        trait Write {        fn write_all(&mut self, bytes: &[u8]) -> Result<(), io::Error>;    }}
