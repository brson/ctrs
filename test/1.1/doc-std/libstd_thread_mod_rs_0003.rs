fn main() {
    #![allow(unused_must_use)]
    use std::thread;
    
    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello, world!");
    });
}
