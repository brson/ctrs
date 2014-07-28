fn main() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
    let mut xs = Nil;
    let ys = xs;
    
    // attempting to use `xs` will result in an error here
    
    xs = Nil;
    
    // `xs` can be used again
}