fn main() {
    // we can just derive a `Copy` implementation
    #[derive(Debug, Copy, Clone)]
    struct Foo;
    
    let x = Foo;
    
    let y = x;
    
    // `y` is a copy of `x`
    
    println!("{:?}", x); // A-OK!
}
