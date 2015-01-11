fn main() {
    let k = 21u;
    
    let x = Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3u);
    
    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42u);
}
