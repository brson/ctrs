fn main() {
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));
    
    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));
}
