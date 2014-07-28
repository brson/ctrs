fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    fn eq<T: PartialEq>(xs: &List<T>, ys: &List<T>) -> bool {
        // Match on the next node in both lists.
        match (xs, ys) {
            // If we have reached the end of both lists, they are equal.
            (&Nil, &Nil) => true,
            // If the current elements of both lists are equal, keep going.
            (&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
                    if x == y => eq(next_xs, next_ys),
            // If the current elements are not equal, the lists are not equal.
            _ => false
        }
    }
    
    let xs = Cons('c', box Cons('a', box Cons('t', box Nil)));
    let ys = Cons('c', box Cons('a', box Cons('t', box Nil)));
    assert!(eq(&xs, &ys));
}