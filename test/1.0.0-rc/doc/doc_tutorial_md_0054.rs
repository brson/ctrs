fn main() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
    let list = Cons(1, box Cons(2, box Cons(3, box Nil)));
}