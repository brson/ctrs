fn main() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }
    fn eq(xs: &List, ys: &List) -> bool {
        // Match on the next node in both lists.
        match (xs, ys) {
            // If we have reached the end of both lists, they are equal.
            (&Nil, &Nil) => true,
            // If the current elements of both lists are equal, keep going.
            (&Cons(x, box ref next_xs), &Cons(y, box ref next_ys))
                    if x == y => eq(next_xs, next_ys),
            // If the current elements are not equal, the lists are not equal.
            _ => false
        }
    }
    
    let xs = Cons(5, box Cons(10, box Nil));
    let ys = Cons(5, box Cons(10, box Nil));
    assert!(eq(&xs, &ys));
}