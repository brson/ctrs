fn main() {
    fn foo(s: &str) {

        // borrow a string for a second

    }

    

    // String implements Deref<Target=str>

    let owned = "Hello".to_string();

    

    // therefore, this works:

    foo(&owned);

}
