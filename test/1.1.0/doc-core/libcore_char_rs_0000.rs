fn main() {
    use std::char;
    
    let c = char::from_u32(10084); // produces `Some(❤)`
    assert_eq!(c, Some('❤'));
}
