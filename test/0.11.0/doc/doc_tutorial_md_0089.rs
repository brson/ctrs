fn main() {
    use std::rc::Rc;
    
    // A fixed-size array allocated in a reference-counted box
    let x = Rc::new([1i, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let y = x.clone(); // a new owner
    let z = x; // this moves `x` into `z`, rather than creating a new owner
    
    assert!(*z == [1i, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    // the variable is mutable, but not the contents of the box
    let mut a = Rc::new([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    a = z;
}