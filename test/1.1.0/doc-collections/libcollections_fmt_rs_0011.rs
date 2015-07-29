fn main() {
    #![allow(unused_must_use)]
    use std::io::Write;
    let mut w = Vec::new();
    write!(&mut w, "Hello {}!", "world");
}
