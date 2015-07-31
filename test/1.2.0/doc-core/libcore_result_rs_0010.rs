fn main() {
    macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => return Err(e) })
    }
}
