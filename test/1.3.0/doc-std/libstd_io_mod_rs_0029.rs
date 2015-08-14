fn main() {
    use std::io;
    use std::io::prelude::*;
    
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    
    // we can't have two `&mut` references to `stdin`, so use a block
    // to end the borrow early.
    let length = {
        let buffer = stdin.fill_buf().unwrap();
    
        // work with buffer
        println!("{:?}", buffer);
    
        buffer.len()
    };
    
    // ensure the bytes we worked with aren't returned again later
    stdin.consume(length);
}
