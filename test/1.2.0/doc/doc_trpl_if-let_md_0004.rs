fn main() {
    let option: Option<i32> = None;
    loop {
    match option {
    Some(x) => println!("{}", x),
    _ => break,
    }
    }
}
