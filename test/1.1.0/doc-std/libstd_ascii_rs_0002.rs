fn main() {
    use std::ascii::AsciiExt;        let ascii = 'A';    let utf8 = '❤';        assert_eq!('a', ascii.to_ascii_lowercase());    assert_eq!('❤', utf8.to_ascii_lowercase());}
