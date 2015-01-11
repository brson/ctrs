fn main() {
    fn sq(x: uint) -> Result<uint, uint> { Ok(x * x) }
    fn err(x: uint) -> Result<uint, uint> { Err(x) }
    
    assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
}
