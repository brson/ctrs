fn main() {
    let mut x: Result<u32, &str> = Ok(7);    match x.iter_mut().next() {        Some(&mut ref mut x) => *x = 40,        None => {},    }    assert_eq!(x, Ok(40));        let mut x: Result<u32, &str> = Err("nothing!");    assert_eq!(x.iter_mut().next(), None);}
