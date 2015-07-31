fn main() {
    let mut range = 0..10;
    
    loop {
    match range.next() {
    Some(x) => {
    println!("{}", x);
    },
    None => { break }
    }
    }
}
