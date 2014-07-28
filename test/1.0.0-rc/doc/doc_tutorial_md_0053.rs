fn main() {
    
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
}