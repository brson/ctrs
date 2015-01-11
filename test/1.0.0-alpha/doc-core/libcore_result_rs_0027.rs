fn main() {
    let optb = 2u;
    let x: Result<uint, &str> = Ok(9u);
    assert_eq!(x.unwrap_or(optb), 9u);
    
    let x: Result<uint, &str> = Err("error");
    assert_eq!(x.unwrap_or(optb), optb);
}
