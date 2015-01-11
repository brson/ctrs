fn main() {
    let x: Result<uint, &str> = Ok(7);
    assert_eq!(x.iter().next(), Some(&7));
    
    let x: Result<uint, &str> = Err("nothing!");
    assert_eq!(x.iter().next(), None);
}
