fn main() {
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}
