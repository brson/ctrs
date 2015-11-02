// ctrs-broken 1.4

fn main() {
    use std::io;
    use std::io::prelude::*;
    
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();
    
    while stdin.read_line(&mut buffer).unwrap() > 0 {
        // work with buffer
        println!("{:?}", buffer);
    
        buffer.clear();
    }
}
