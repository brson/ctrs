fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    fn prepend<T>(xs: List<T>, value: T) -> List<T> {
        Cons(value, box xs)
    }
    let mut xs = Nil; // Unknown type! This is a `List<T>`, but `T` can be anything.
    xs = prepend(xs, 10i); // Here the compiler infers `xs`'s type as `List<int>`.
    xs = prepend(xs, 15i);
    xs = prepend(xs, 20i);
}