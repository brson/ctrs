enum List { Nil, Cons(uint, Box<List>) }

fn is_sorted(list: &List) -> bool {
    match *list {
        Nil | Cons(_, box Nil) => true,
        Cons(x, ref r @ box Cons(y, _)) => (x <= y) && is_sorted(&**r)
    }
}

fn main() {
    let a = Cons(6, box Cons(7, box Cons(42, box Nil)));
    assert!(is_sorted(&a));
}

