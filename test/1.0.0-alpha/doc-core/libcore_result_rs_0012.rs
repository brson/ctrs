fn main() {
    let x: Result<int, &str> = Ok(-3);
    assert_eq!(x.is_err(), false);
    
    let x: Result<int, &str> = Err("Some error message");
    assert_eq!(x.is_err(), true);
}
