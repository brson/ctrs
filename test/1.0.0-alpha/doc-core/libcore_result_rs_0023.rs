fn main() {
    let x: Result<uint, &str> = Ok(2);
    let y: Result<&str, &str> = Err("late error");
    assert_eq!(x.and(y), Err("late error"));
    
    let x: Result<uint, &str> = Err("early error");
    let y: Result<&str, &str> = Ok("foo");
    assert_eq!(x.and(y), Err("early error"));
    
    let x: Result<uint, &str> = Err("not a 2");
    let y: Result<&str, &str> = Err("late error");
    assert_eq!(x.and(y), Err("not a 2"));
    
    let x: Result<uint, &str> = Ok(2);
    let y: Result<&str, &str> = Ok("different result type");
    assert_eq!(x.and(y), Ok("different result type"));
}
