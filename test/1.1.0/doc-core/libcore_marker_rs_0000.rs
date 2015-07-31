fn main() {
    #[derive(Debug)]
    struct Foo;
    
    let x = Foo;
    
    let y = x;
    
    // `x` has moved into `y`, and so cannot be used
    
    // println!("{:?}", x); // error: use of moved value
}
