fn main() {
    let x: Result<uint, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));
    
    let x: Result<uint, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);
}
