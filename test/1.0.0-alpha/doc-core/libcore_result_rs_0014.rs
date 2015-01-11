fn main() {
    let x: Result<uint, &str> = Ok(2);
    assert_eq!(x.err(), None);
    
    let x: Result<uint, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));
}
