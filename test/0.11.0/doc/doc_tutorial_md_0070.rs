fn main() {
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil
    }
    impl<T: PartialEq> PartialEq for List<T> {
        fn eq(&self, ys: &List<T>) -> bool {
            // Match on the next node in both lists.
            match (self, ys) {
                // If we have reached the end of both lists, they are equal.
                (&Nil, &Nil) => true,
                // If the current elements of both lists are equal, keep going.
                (&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
                        if x == y => next_xs == next_ys,
                // If the current elements are not equal, the lists are not equal.
                _ => false
            }
        }
    }
    
    let xs = Cons(5i, box Cons(10i, box Nil));
    let ys = Cons(5i, box Cons(10i, box Nil));
    // The methods below are part of the PartialEq trait,
    // which we implemented on our linked list.
    assert!(xs.eq(&ys));
    assert!(!xs.ne(&ys));
    
    // The PartialEq trait also allows us to use the shorthand infix operators.
    assert!(xs == ys);    // `xs == ys` is short for `xs.eq(&ys)`
    assert!(!(xs != ys)); // `xs != ys` is short for `xs.ne(&ys)`
}