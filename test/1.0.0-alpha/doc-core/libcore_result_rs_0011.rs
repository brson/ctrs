fn main() {
    let x: Result<int, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    
    let x: Result<int, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}
