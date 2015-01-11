fn main() {
    fn count(x: &str) -> uint { x.len() }
    
    assert_eq!(Ok(2u).unwrap_or_else(count), 2u);
    assert_eq!(Err("foo").unwrap_or_else(count), 3u);
}
