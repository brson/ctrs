fn main() {
    let x: Result<uint, &str> = Ok(2);
    assert_eq!(x.as_ref(), Ok(&2));
    
    let x: Result<uint, &str> = Err("Error");
    assert_eq!(x.as_ref(), Err(&"Error"));
}
