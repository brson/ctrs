fn main() {
    let x: Result<uint, &str> = Ok(5);
    let v: Vec<uint> = x.into_iter().collect();
    assert_eq!(v, vec![5u]);
    
    let x: Result<uint, &str> = Err("nothing!");
    let v: Vec<uint> = x.into_iter().collect();
    assert_eq!(v, vec![]);
}
