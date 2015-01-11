fn main() {
    let x = Some(2u);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);
    
    let x: Option<uint> = None;
    let y = Some("foo");
    assert_eq!(x.and(y), None);
    
    let x = Some(2u);
    let y = Some("foo");
    assert_eq!(x.and(y), Some("foo"));
    
    let x: Option<uint> = None;
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);
}
