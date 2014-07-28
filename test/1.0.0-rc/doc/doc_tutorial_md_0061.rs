fn main() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
    
    fn prepend(xs: List, value: u32) -> List {
        Cons(value, box xs)
    }
    
    let mut xs = Nil;
    xs = prepend(xs, 1);
    xs = prepend(xs, 2);
    xs = prepend(xs, 3);
}