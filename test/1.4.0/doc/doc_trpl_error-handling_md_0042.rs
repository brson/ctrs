fn main() {
    macro_rules! try {
        ($e:expr) => (match $e {
            Ok(val) => val,
            Err(err) => return Err(::std::convert::From::from(err)),
        });
    }
}
