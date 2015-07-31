fn main() {
    use std::ascii::AsciiExt;
    
    let ascii1 = 'A';
    let ascii2 = 'a';
    let ascii3 = 'A';
    let ascii4 = 'z';
    
    assert_eq!(true, ascii1.eq_ignore_ascii_case(&ascii2));
    assert_eq!(true, ascii1.eq_ignore_ascii_case(&ascii3));
    assert_eq!(false, ascii1.eq_ignore_ascii_case(&ascii4));
}
