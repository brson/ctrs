fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    fn prepend<T>(xs: List<T>, value: T) -> List<T> {
        Cons(value, box xs)
    }
}