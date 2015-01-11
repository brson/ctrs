fn main() {
    #![allow(unused_must_use)]
    use std::io;
    
    let mut w = Vec::new();
    write!(&mut w as &mut io::Writer, "Hello {}!", "world");
}
