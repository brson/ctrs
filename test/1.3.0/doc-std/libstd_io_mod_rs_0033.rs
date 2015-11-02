// ctrs-broken 1.4

fn main() {
    use std::io;
    use std::io::prelude::*;
    
    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
