fn main() {
    use std::rc::Rc;

    

    fn foo(s: &str) {

        // borrow a string for a second

    }

    

    // String implements Deref<Target=str>

    let owned = "Hello".to_string();

    let counted = Rc::new(owned);

    

    // therefore, this works:

    foo(&counted);

}
