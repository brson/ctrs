fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    fn prepend<T>(xs: List<T>, value: T) -> List<T> {
        Cons(value, box xs)
    }
    let mut xs: List<int> = Nil::<int>;
    xs = prepend::<int>(xs, 10);
    xs = prepend::<int>(xs, 15);
    xs = prepend::<int>(xs, 20);
}