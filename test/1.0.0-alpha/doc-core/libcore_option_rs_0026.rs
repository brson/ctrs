fn main() {
    fn nobody() -> Option<&'static str> { None }
    fn vikings() -> Option<&'static str> { Some("vikings") }
    
    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);
}
