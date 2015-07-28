fn main() {
    use std::ascii::AsciiExt;
    
    let ascii = 'a';
    let utf8 = '❤';
    
    assert_eq!(true, ascii.is_ascii());
    assert_eq!(false, utf8.is_ascii())
}
