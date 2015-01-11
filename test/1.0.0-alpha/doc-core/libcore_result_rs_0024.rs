fn main() {
    fn sq(x: uint) -> Result<uint, uint> { Ok(x * x) }
    fn err(x: uint) -> Result<uint, uint> { Err(x) }
    
    assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
    assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
    assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
    assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));
}
