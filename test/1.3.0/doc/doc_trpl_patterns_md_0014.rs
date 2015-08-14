fn main() {
    let x = 5;
    
    match x {
        ref r => println!("Got a reference to {}", r),
    }
}
