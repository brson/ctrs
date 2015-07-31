fn main() {
    use std::char;
    
    let c = char::from_digit(4, 10);
    
    assert_eq!(c, Some('4'));
}
