#[deriving(Show)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<int> = Cons(1, box Cons(2, box Cons(3, box Nil)));
    println!("{}", list);
}
