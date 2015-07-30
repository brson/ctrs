fn main() {
    use std::ascii::AsciiExt;
    
    let ascii = 'a';
    let utf8 = '❤';
    
    assert_eq!('A', ascii.to_ascii_uppercase());
    assert_eq!('❤', utf8.to_ascii_uppercase());
}
