fn main() {
    let x: Result<uint, &str> = Ok(2);
    let y: Result<uint, &str> = Err("late error");
    assert_eq!(x.or(y), Ok(2));
    
    let x: Result<uint, &str> = Err("early error");
    let y: Result<uint, &str> = Ok(2);
    assert_eq!(x.or(y), Ok(2));
    
    let x: Result<uint, &str> = Err("not a 2");
    let y: Result<uint, &str> = Err("late error");
    assert_eq!(x.or(y), Err("late error"));
    
    let x: Result<uint, &str> = Ok(2);
    let y: Result<uint, &str> = Ok(100);
    assert_eq!(x.or(y), Ok(2));
}
