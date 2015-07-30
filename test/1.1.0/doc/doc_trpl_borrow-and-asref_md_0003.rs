fn main() {
    use std::borrow::Borrow;
    use std::fmt::Display;
    
    fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
    }
    
    let mut i = 5;
    
    foo(&i);
    foo(&mut i);
}
