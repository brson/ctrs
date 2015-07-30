fn main() {
    fn foo(s: &[i32]) {
    // borrow a slice for a second
    }
    
    // Vec<T> implements Deref<Target=[T]>
    let owned = vec![1, 2, 3];
    
    foo(&owned);
}
