fn main() {
    let x: Result<uint, &str> = Err("emergency failure");
    assert_eq!(x.unwrap_err(), "emergency failure");
}
