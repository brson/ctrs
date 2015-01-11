fn main() {
    let mut x: Result<uint, &str> = Ok(7);
    match x.iter_mut().next() {
        Some(&mut ref mut x) => *x = 40,
        None => {},
    }
    assert_eq!(x, Ok(40));
    
    let mut x: Result<uint, &str> = Err("nothing!");
    assert_eq!(x.iter_mut().next(), None);
}
