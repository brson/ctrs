fn main() {
    let x: Result<uint, &str> = Ok(2u);
    assert_eq!(x.unwrap(), 2u);
}
