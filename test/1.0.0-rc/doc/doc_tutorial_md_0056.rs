fn main() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
    let xs = Cons(1, box Cons(2, box Cons(3, box Nil)));
    let ys = xs; // copies `Cons(u32, pointer)` shallowly
}