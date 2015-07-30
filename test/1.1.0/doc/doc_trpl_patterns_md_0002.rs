fn main() {
    let x = 1;
    
    match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
    }
}
