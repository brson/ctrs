fn main() {
    fn mutate(r: &mut Result<int, int>) {
        match r.as_mut() {
            Ok(&mut ref mut v) => *v = 42,
            Err(&mut ref mut e) => *e = 0,
        }
    }
    
    let mut x: Result<int, int> = Ok(2);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 42);
    
    let mut x: Result<int, int> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap_err(), 0);
}
