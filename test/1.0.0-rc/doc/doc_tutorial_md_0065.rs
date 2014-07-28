fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
}