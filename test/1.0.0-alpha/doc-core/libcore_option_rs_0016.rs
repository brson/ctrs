fn main() {
    let x = Some("foo");
    assert_eq!(x.map_or(42u, |v| v.len()), 3u);
    
    let x: Option<&str> = None;
    assert_eq!(x.map_or(42u, |v| v.len()), 42u);
}
