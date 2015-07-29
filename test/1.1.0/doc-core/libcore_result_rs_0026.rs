fn main() {
    let optb = 2;    let x: Result<u32, &str> = Ok(9);    assert_eq!(x.unwrap_or(optb), 9);        let x: Result<u32, &str> = Err("error");    assert_eq!(x.unwrap_or(optb), optb);}
