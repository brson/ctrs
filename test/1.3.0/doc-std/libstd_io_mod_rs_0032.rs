fn main() {
    use std::io;
    use std::io::prelude::*;
    
    let stdin = io::stdin();
    
    for content in stdin.lock().split(b',') {
        println!("{:?}", content.unwrap());
    }
}
